#include <condition_variable>
#include <deque>
#include <functional>
#include <future>
#include <memory>
#include <mutex>
#include <thread>
#include <type_traits>
#include <utility>
#include <vector>

class ThreadPool {
private:
#ifdef __cpp_lib_move_only_function
  template <typename Signature>
  using MoveOnlyFunction = std::move_only_function<Signature>;
#else
  // A simple implementation of MoveOnlyFunction
  template <typename Signature> class MoveOnlyFunction;

  template <typename Ret, typename... Args>
  class MoveOnlyFunction<Ret(Args...)> {
  private:
    class Base {
    public:
      virtual ~Base() = default;
      virtual auto operator()(Args &&...args) -> Ret;
    };

    template <typename Callable> class Derived : public Base {
    private:
      Callable func_;

    public:
      explicit Derived(Callable &&func) : func_(std::forward<Callable>(func)) {}
      auto operator()(Args &&...args) -> Ret final {
        func_(std::forward<Args>(args)...);
      }
    };

    std::unique_ptr<Base> func_impl_;

  public:
    explicit MoveOnlyFunction() = default;

    template <typename Callable>
    explicit MoveOnlyFunction(Callable &&func) // NOLINT
        : func_impl_(std::make_unique<Derived<Callable>>(
              std::forward<Callable>(func))) {}

    MoveOnlyFunction(const MoveOnlyFunction &) = delete;
    auto operator=(const MoveOnlyFunction &) = delete;
    MoveOnlyFunction(MoveOnlyFunction &&other) noexcept {
      func_impl_ = other.func_impl_;
      other.func_impl_ = nullptr;
    }
    auto operator=(MoveOnlyFunction &&other) noexcept -> MoveOnlyFunction & {
      if (this != &other) {
        std::swap(other.func_impl_, func_impl_);
      }
      return *this;
    }

    auto operator()(Args &&...args) -> Ret {
      (*func_impl_)(std::forward<Args>(args)...);
    }
  };
#endif

  std::deque<MoveOnlyFunction<void()>> tasks_{};
  std::vector<std::thread> threads_{};
  std::mutex lock_{};
  std::condition_variable cv_{};
  bool stopped_ = false;

public:
  explicit ThreadPool(std::size_t threads) {
    for (std::size_t i = 0; i < threads; ++i) {
      threads_.emplace_back([this]() {
        while (true) {
          auto task = MoveOnlyFunction<void()>{}; // benchmark inside or outside
          {
            auto ulock = std::unique_lock<std::mutex>{lock_};
            cv_.wait(ulock, [this]() {
              return this->stopped_ || !this->tasks_.empty();
            });

            if (stopped_) {
              return;
            }

            task = std::move(tasks_.front());
            tasks_.pop_front();
          }
          task();
        }
      });
    }
  }
  ~ThreadPool() {
    if (!stopped_) {
      Stop();
    }
  }
  ThreadPool(const ThreadPool &) = delete;
  ThreadPool(ThreadPool &&) = delete;
  auto operator=(ThreadPool) -> ThreadPool & = delete;

  auto Stop() -> void {
    stopped_ = true;
    cv_.notify_all();
    for (auto &thread : threads_) {
      thread.join();
    }
  }

  template <typename Callable, typename... Args>
  auto Enqueue(Callable &&func, Args &&...args) {
    using Return_Type = std::invoke_result_t<Callable, Args...>;

    auto promise = std::promise<Return_Type>{};
    const auto future = promise.get_future();
    {
      const auto guard = std::lock_guard<std::mutex>{lock_};
      tasks_.emplace_back(
          [promise = std::move(promise),
           task = std::bind(std::forward<Callable>(func),
                            std::forward<Args>(args)...)]() mutable {
            if constexpr (std::is_void_v<Return_Type>) {
              std::invoke(task);
              promise.set_value();
            } else {
              promise.set_value(std::invoke(task));
            }
          });
    }
    cv_.notify_one();
    return future;
  }
};