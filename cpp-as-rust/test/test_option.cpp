#include "catch_amalgamated.hpp"
#include "csr.h"

using namespace csr;

struct ATestClass
{
    u8 x;
};

TEST_CASE("Should be able to construct Option via Some and None", "[Option]")
{
    Option<usize>::Some(0);
    Option<i32>::None();
    Option<ATestClass>::Some({});
}

TEST_CASE("is_some and is_none should return correct boolean value", "[Option]")
{
    auto op1 = Option<usize>::Some(0);
    REQUIRE(op1.is_some());
    REQUIRE_FALSE(op1.is_none());

    auto op2 = Option<i32>::None();
    REQUIRE(op2.is_none());
    REQUIRE_FALSE(op2.is_some());

    auto op3 = Option<ATestClass>::Some({});
    REQUIRE(op3.is_some());
    REQUIRE_FALSE(op3.is_none());
}

TEST_CASE("Should be able to move construct/assign from existing Option", "[Option]")
{
    Option<ATestClass> option1 = Option<ATestClass>::Some({});
    REQUIRE(option1.is_some());

    Option<ATestClass> option2 = std::move(option1);
    REQUIRE(option2.is_some());

    Option<ATestClass> option3 = Option<ATestClass>::None();
    REQUIRE(option3.is_none());
    option3 = std::move(option2);
    REQUIRE(option3.is_some());
}

TEST_CASE("Should be able to observe the Option", "[Option]"){
    auto op1 = Option<usize>::Some(0);
    REQUIRE(op1.unwrap() == 0);
    REQUIRE(op1.expect("Test Expect") == 0);

    auto op2 = Option<ATestClass>::Some({});
    REQUIRE(op2.unwrap().x == 0);
    REQUIRE(op2.expect("Test Expect").x == 0);
}

TEST_CASE("Should throw bad_option_access if observing None", "[Option]"){
    auto op1 = Option<usize>::None();
    REQUIRE_THROWS_AS(op1.unwrap(), std::bad_optional_access);
    REQUIRE_THROWS_AS(op1.expect("Expect to Throw"), std::bad_optional_access);

    auto op2 = Option<ATestClass>::None();
    REQUIRE_THROWS_AS(op2.unwrap(), std::bad_optional_access);
    REQUIRE_THROWS_AS(op2.expect("Expect to Throw"), std::bad_optional_access);
}

TEST_CASE("unwrap_or Should return the correct value", "[Option]"){
    auto op1 = Option<usize>::None();
    REQUIRE(op1.unwrap_or(1) == 1);

    auto op2 = Option<ATestClass>::Some({1});
    REQUIRE(op2.unwrap_or({0}).x == 1);
}
