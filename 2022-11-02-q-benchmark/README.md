# q-benchmark

q-benchmark is a command-line benchmark tool inspired by an awesome command-line benchmark tool [hyperfine](https://github.com/sharkdp/hyperfine).

This tool uses about `250` lines of code to build a simple replica of hyperfine.

## Features

- Support for any shell commands
- Support for defining min/max number of test runs
- Report mean/std/range for tests
- warmup before running the benchmark

## Usage

q-benchmark only supports macOS/Linux now.

To use q-benchmark:
1. Make sure your compiler supports `c++17`.
2. Run `make`.
3. Run benchmark by using `./bin/release/qbench [options] "command1" "command2" ...`
    - Options:
      - `-w` times: assign the number of warmup function calls to the times. The default number of warmup function calls is `3`.
      - `-l` lower: lower bound of the number of test runs. The default number is 100.
      - `-h` upper: upper bound of the number of test runs. The default number is 1000.

## Example

To benchmark the `sleep` function, you can run:

```sh
./bin/release/qbench -w 3 -l 1 -h 5 "sleep 0.3" "sleep 0.3"
```

Then q-benchmark will output the statistical results:

```
Benchmark: sleep (3 runs)
  Time: 306.64 ms(mean) ± 1.76368 ms(std)
  Range: 304.717 ms(min) … 308.977 ms(max)

Benchmark: sleep (5 runs)
  Time: 308.31 ms(mean) ± 0.991041 ms(std)
  Range: 306.783 ms(min) … 309.333 ms(max)
```

## Known Issues

The parser simply uses whitespace to separate the command line arguments. Therefore, it may fail under some circumstances.