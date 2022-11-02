#include "catch_amalgamated.hpp"
#include "csr.h"

TEST_CASE("Size of the type Should be correct", "[Types]"){
    REQUIRE(sizeof(u8) == 1);
    REQUIRE(sizeof(u16) == 2);
    REQUIRE(sizeof(u32) == 4);
    REQUIRE(sizeof(u64) == 8);
    REQUIRE(sizeof(i8) == 1);
    REQUIRE(sizeof(i16) == 2);
    REQUIRE(sizeof(i32) == 4);
    REQUIRE(sizeof(i64) == 8);
    REQUIRE(sizeof(f32) == 4);
    REQUIRE(sizeof(f64) == 8);
}