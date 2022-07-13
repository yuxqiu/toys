#define CATCH_CONFIG_MAIN
#include "catch.hpp"

#include <cmath>
#include "mymatrix.h"

using namespace mx;
using namespace std;

TEST_CASE("Size works correctly", "[size]")
{
    Matrix m = eye(3);

    REQUIRE(m.size().row == 3);
    REQUIRE(m.size().col == 3);
    REQUIRE(m.size(1) == 3);
    REQUIRE(m.size(2) == 3);

    Matrix a = randn(5, 7);
    REQUIRE(a.size().row == 5);
    REQUIRE(a.size().col == 7);
    REQUIRE(a.size(1) == 5);
    REQUIRE(a.size(2) == 7);
}

TEST_CASE("Equal works correctly", "[equal]")
{
    Matrix m = randn(5, 7);
    REQUIRE(m == m);
}

TEST_CASE("Assignment works correctly", "[assign]")
{
    Matrix m = randn(5, 7);
    Matrix z = m;

    REQUIRE(m == z);
}

TEST_CASE("Append works correctly", "[append]")
{
    Matrix r1 = randn(5, 7);
    Matrix r2 = randn(3, 7);
    Matrix r3 = randn(5, 10);

    Matrix s1 = Matrix::append(r1, r3, 2);
    Matrix s2 = Matrix::append(r1, r2, 1);

    assert(s1.size(1) == 5);
    assert(s1.size(2) == 17);
    assert(s2.size(1) == 8);
    assert(s2.size(2) == 7);

    Matrix i = eye(3);
    Matrix z = zeros(3, 3);

    Matrix s3 = Matrix::append(i, z, 2);
    Matrix s4 = Matrix::append(z, i, 2);
    Matrix s5 = Matrix::append(s3, s4, 1);

    REQUIRE(s5 == eye(6));
}

TEST_CASE("Inverse works correctly", "[inverse]")
{
    Matrix e = eye(3);

    REQUIRE(e == e.inv());

    double e1[2][2] = {{-4, 3}, {3, -2}};
    double e2[2][2] = {{2, 3}, {3, 4}};

    int size = 2;
    Matrix s1 = zeros(size, size);
    for (int i = 0; i < size; ++i)
    {
        for (int j = 0; j < size; ++j)
            s1(i, j) = e1[i][j];
    }

    Matrix s2 = zeros(size, size);
    for (int i = 0; i < size; ++i)
    {
        for (int j = 0; j < size; ++j)
            s2(i, j) = e2[i][j];
    }

    REQUIRE(s1.inv() == s2);
    REQUIRE(s1.inv().inv() == s1);
}

TEST_CASE("Operator works correctly", "[op]")
{
    Matrix i = eye(5);
    REQUIRE(i * 5 / 5 == i);

    Matrix s1 = eye(1);
    REQUIRE(s1 * -1 + 1 == zeros(1, 1));
    REQUIRE(s1 * -1 + 2 == ones(1, 1));

    REQUIRE(ones(1, 1) - 1 == zeros(1, 1));
    REQUIRE(ones(5, 7) - 1 == zeros(5, 7));

    REQUIRE(zeros(3, 3) + zeros(3, 3) == zeros(3, 3));
    REQUIRE(ones(5, 5) * 3 == ones(5, 5) + ones(5, 5) + ones(5, 5) + zeros(5, 5));

    REQUIRE((ones(5, 7) ^ 2) == ones(5, 7));
}

TEST_CASE("Determinant works correctly", "[det]")
{
    Matrix i = eye(3);
    REQUIRE(i.det() == 1);

    Matrix s1 = ones(3, 3);
    REQUIRE(s1.det() == 0);

    Matrix s2 = zeros(3, 3);
    REQUIRE(s2.det() == 0);

    Matrix s3 = randn(1, 1);
    REQUIRE(s3.det() == s3(0, 0));

    REQUIRE((s1 * 5).det() == 0);
    REQUIRE((i * 10).det() == 1000);

    double e1[3][3] = {{1, 2, 3}, {4, 5, 6}, {7, 8, 9}};

    int size = 3;
    Matrix s4 = zeros(size, size);
    for (int i = 0; i < size; ++i)
    {
        for (int j = 0; j < size; ++j)
            s4(i, j) = e1[i][j];
    }

    REQUIRE(s4.det() == 0);
}

TEST_CASE("Transpose works correctly", "[trans]")
{
    Matrix i = eye(3);
    REQUIRE(i == i.trans());
    REQUIRE(zeros(5, 7) == zeros(7, 5).trans());
    REQUIRE(ones(5, 5) == ones(5, 5).trans());

    Matrix q = randn(3, 17);
    REQUIRE(q == q.trans().trans());
}

TEST_CASE("Maps works correctly", "[maps]")
{
    REQUIRE(ones(3, 7).map(log) == zeros(3, 7));
    REQUIRE(ones(5, 7).map(sqrt) == ones(5, 7));
}

TEST_CASE("Sum works correctly", "[sum]")
{
    REQUIRE(ones(3, 3).sum(1) == ones(1, 3) * 3);
    REQUIRE(eye(3).sum(2) == ones(3, 1));
}

TEST_CASE("Logical operators work correctly", "[logic]")
{
    REQUIRE((ones(5, 7) == 0) == zeros(5, 7));
    REQUIRE((eye(3) <= 1) == ones(3, 3));
    REQUIRE((zeros(5, 7) >= 0) == ones(5, 7));
    REQUIRE((randn(7, 14) <= 1) == ones(7, 14));
    REQUIRE((eye(3) > 1) == zeros(3, 3));
    REQUIRE((ones(5, 7) < 1) == zeros(5, 7));
}

TEST_CASE("Strassen Algorithm works correctly", "[strassen]"){
    // The strassen algorithm may cause errors in calculations.
    // So, do not compare the multiplication result of the randn matrices.
    REQUIRE((eye(3) * eye(3)) == Matrix::strassen(eye(3), eye(3)));
}