# MyMatrix

MyMatrix is a C++ implementation of a simple matrix library that supports basic Matrix operations, including Matrix inverse, transpose, and multiplication.

## Usage

You can create a matrix via the following methods:

```c++
// This will create a 3*3 identity matrix.
Matrix x = eye(3);

// This will copy (deepcopy) x to y.
Matrix y = x;

// This will create a 5*7 matrix with all ones.
Matrix o = ones(5, 7);

// This will create a 3*10 matrix with all zeros.
Matrix z = zeros(3, 10);

// This will create a 3*7 matrix with random values.
Matrix r = randn(3, 7);

// This will create a 6*4 empty matrix with uninitialised values.
Matrix e(6, 4);
```

With the matrix object, you can use the following methods:

```c++
Matrix x = randn(5, 5);

// This will return the inverse matrix of x.
x.inv();

// This will return the Moore-Penrose Pseudoinverse of x.
x.pinv();

// This will return the determinant of x.
x.det();

// This will return the transpose of x.
x.trans();

// Map a function f to every element of the matrix x and return the new matrix. x will remain unchanged.
x.map(f);

// This will print the matrix to stdout.
x.display();

```

Also, the following operators are overloaded to perform their corresponding operations in the matrix operations:

```c++
Matrix a = rand(3, 7);
Matrix b = randn(7, 3);
Matrix c = randn(7, 3);
double factor = 2;

// a * b will return a 3*3 matrix that is a result of matrix multiplication of a and b
// b + c will return the result of adding the corresponding elements of the two matrices.
// factor * c will return a matrix that is a result of matrix scalar multiplication of factor and c
// factor + c adds the factor to every element of the c and return this new matrix. c will remain unchanged.
```

Logical operators are also supported by the matrix library:

```c++
Matrix a = eye(3);

// The matrix b will be one everywhere except for the diagonal elements.
Matrix b = a == 0;

// The matrix c will be one everywhere.
Matrix c = a <= 1;

// >=, >, < are also supported.
```

## Test

The library has been tested by using [Catch2](https://github.com/catchorg/Catch2). Its basic functionality currently passes all the tests listed in `./test/test.cpp`. However, these tests are not guaranteed to be comprehensive. Therefore, if there are any issues with the library, feel free to fix them by pushing a request.

## Contribute

We welcome you to contribute to this project in the following ways:
1. Edit the existing algorithm to make it faster.
2. Add additional features.
3. Add more test cases.

## New Features

- [x] Implementation of pseudo-inverse
- [x] Implementation of faster matrix multiplication algorithm
- [ ] Faster Implementation: See [BLAS-level CPU Performance in 100 Lines of C](https://cs.stanford.edu/people/shadjis/blas.html)

## Notes

1. The Moore-Penrose Pseudoinverse are implemented based on the fact that

    $$ A^+ = \lim_{\delta \to 0} (A^T A + \delta^2 I) A^T $$

    In the library, the delta value is chosen to be 0.0001. This value can be reduced to obtain more accurate results, but the issue of precision needs to be looked at carefully.

2. Although Strassen's algorithm is asymptotically faster than the traditional matrix multiplication, it's not the default implementation of the matrix multiplication in MyMatrix library. It's also not suggested to use this algorithm in real-life applications. For more detailed reasons, you can refer to [Strassen algorithm: Asymptotic complexity](https://en.wikipedia.org/wiki/Strassen_algorithm#Asymptotic_complexity).