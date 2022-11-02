#include "catch_amalgamated.hpp"
#include "csr.h"

using namespace csr;

struct ATestClass
{
    u8 x;
};

TEST_CASE("Should be able to construct Result via Ok and Err", "[Result]")
{
    Result<usize, std::exception>::Ok(0).unwrap();
    Result<i32, std::exception>::Err({}).unwrap_err();
    Result<ATestClass, std::exception>::Ok({}).unwrap();
}

TEST_CASE("is_ok and is_err should return correct boolean value", "[Result]")
{
    auto res1 = Result<usize, std::exception>::Ok(0);
    REQUIRE(res1.is_ok());
    REQUIRE_FALSE(res1.is_err());

    auto res2 = Result<i32, std::exception>::Err({});
    REQUIRE(res2.is_err());
    REQUIRE_FALSE(res2.is_ok());

    auto res3 = Result<ATestClass, std::exception>::Ok({});
    REQUIRE(res3.is_ok());
    REQUIRE_FALSE(res3.is_err());
}

TEST_CASE("Should be able to move construct/assign from existing Result", "[Result]")
{
    Result<ATestClass, std::exception> res1 = Result<ATestClass, std::exception>::Ok({});
    REQUIRE(res1.is_ok());

    Result<ATestClass, std::exception> res2 = std::move(res1);
    REQUIRE(res2.is_ok());

    Result<ATestClass, std::exception> res3 = Result<ATestClass, std::exception>::Err({});
    REQUIRE(res3.is_err());
    res3 = std::move(res2);
    REQUIRE(res3.is_ok());
}

TEST_CASE("Should observe Ok Result via unwrap and expect", "[Result]")
{
    auto res1 = Result<usize, std::exception>::Ok(0);
    REQUIRE(res1.unwrap() == 0);
    REQUIRE(res1.expect("Test Expect") == 0);

    auto res2 = Result<ATestClass, std::exception>::Ok({});
    REQUIRE(res2.unwrap().x == 0);
    REQUIRE(res2.expect("Test Expect").x == 0);
}

TEST_CASE("Should throw bad_option_access if observing Err", "[Result]")
{
    auto res1 = Result<usize, std::exception>::Err({});
    REQUIRE_THROWS_AS(res1.unwrap(), std::exception);
    REQUIRE_THROWS_AS(res1.expect("Expect to Throw"), std::exception);

    auto res2 = Result<ATestClass, std::exception>::Err({});
    REQUIRE_THROWS_AS(res2.unwrap(), std::exception);
    REQUIRE_THROWS_AS(res2.expect("Expect to Throw"), std::exception);
}

TEST_CASE("Should observe Err Result via unwrap_err and expect_err", "[Result]")
{
    auto res1 = Result<usize, u8>::Err('a');
    REQUIRE(res1.unwrap_err() == 'a');
    REQUIRE(res1.expect_err("Test Expect") == 'a');

    auto res2 = Result<ATestClass, std::string>::Err("Hello World");
    REQUIRE(res2.unwrap_err() == std::string{"Hello World"});
    REQUIRE(res2.expect_err("Test Expect") == std::string{"Hello World"});
}

TEST_CASE("unwrap_or Should return the correct value", "[Result]")
{
    auto res1 = Result<usize, std::exception>::Err({});
    REQUIRE(res1.unwrap_or(1) == 1);

    auto res2 = Result<ATestClass, std::exception>::Ok({1});
    REQUIRE(res2.unwrap_or({0}).x == 1);
}

TEST_CASE("Should convert to an Option", "[Result]")
{
    auto res1 = Result<usize, std::exception>::Ok(0);
    auto op1 = res1.ok();
    REQUIRE(op1.is_some());
    REQUIRE(op1.unwrap() == res1.unwrap());

    auto res2 = Result<i32, std::exception>::Err({});
    auto op2 = res2.ok();
    REQUIRE(op2.is_none());

    auto res3 = Result<ATestClass, std::exception>::Ok({});
    auto op3 = res3.ok();
    REQUIRE(op3.is_some());
    REQUIRE(op3.unwrap().x == res3.unwrap().x);
}
