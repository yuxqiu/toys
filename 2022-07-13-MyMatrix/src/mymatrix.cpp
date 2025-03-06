#include "mymatrix.h"
#include <cstdio>
#include <random>
#include <cmath>
#include <functional>
#include <stdexcept>

static std::default_random_engine generator;
static std::uniform_real_distribution<double> distribution(0, 1);
static auto gen = std::bind(distribution, generator);

double **mx::Matrix::allocate(int row, int col)
{
    double **ptr = new double *[row];
    for (int i = 0; i < row; ++i)
    {
        ptr[i] = new double[col];
    }

    return ptr;
}

void mx::Matrix::freeResources()
{
    if (matrix == nullptr)
        return;

    for (int i = 0; i < row; ++i)
    {
        delete[] matrix[i];
    }
    delete[] matrix;
}

void mx::Matrix::exchangeRow(int r1, int r2)
{
    if (r1 < 0 || r1 >= row || r2 < 0 || r2 >= row)
        throw std::invalid_argument("r1 or r2 is out of range.");

    double *temp = matrix[r1];
    matrix[r1] = matrix[r2];
    matrix[r2] = temp;
}

mx::Matrix::Matrix(int row, int col, double **matrix) : row(row), col(col), matrix(matrix)
{
}

mx::Matrix::Matrix(int row, int col) : row(row), col(col)
{
    matrix = allocate(this->row, this->col);
}

mx::Matrix::Matrix(const mx::Matrix &m) : row(m.row), col(m.col)
{
    matrix = allocate(this->row, this->col);
    for (int i = 0; i < this->row; ++i)
    {
        for (int j = 0; j < this->col; ++j)
        {
            matrix[i][j] = m.matrix[i][j];
        }
    }
}

mx::Matrix::~Matrix()
{
    freeResources();
}

mx::Tuple mx::Matrix::size() const
{
    return {row, col};
}

int mx::Matrix::size(int axis) const
{
    if (axis == 1)
        return row;
    else if (axis == 2)
        return col;

    throw std::invalid_argument("axis must be 1 or 2.");
}

mx::Matrix mx::Matrix::inv() const
{
    if (row != col)
        throw std::invalid_argument("Matrix must be a square matrix.");

    Matrix n = append(*this, eye(row), 2);

    for (int i = 0; i < n.row; ++i)
    {
        if (n.matrix[i][i] == 0)
        {
            for (int row_i = n.row - 1; row_i > i; --row_i)
            {
                if (n.matrix[row_i][i] != 0)
                {
                    n.exchangeRow(i, row_i);
                    break;
                }
            }

            // Case: if there is no row exchange
            if (n.matrix[i][i] == 0)
                throw std::invalid_argument("The matrix is singular.");
        }

        for (int j = 0; j < n.row; ++j)
        {
            if (i == j)
                continue;

            double factor = n.matrix[j][i] / n.matrix[i][i];
            for (int col_j = 0; col_j < n.col; ++col_j)
            {
                n.matrix[j][col_j] -= factor * n.matrix[i][col_j];
            }
        }
    }

    for (int i = 0; i < n.row; ++i)
    {
        double factor = 1 / n.matrix[i][i];
        for (int j = 0; j < n.col; ++j)
        {
            n.matrix[i][j] *= factor;
        }
    }

    return n(0, n.row - 1, col, n.col - 1);
}

mx::Matrix mx::Matrix::pinv() const
{
    Matrix mt = this->trans();
    return (mt * (*this) + delta * delta * eye(col)).inv() * mt;
}

double mx::Matrix::det() const
{
    if (row != col)
        throw std::invalid_argument("Matrix must be a square matrix.");

    Matrix n = *this;

    int numberOfExchanges = 0;

    for (int i = 0; i < n.row; ++i)
    {
        if (n.matrix[i][i] == 0)
        {
            for (int row_i = n.row - 1; row_i > i; --row_i)
            {
                if (n.matrix[row_i][i] != 0)
                {
                    n.exchangeRow(i, row_i);
                    break;
                }
            }

            // Case: if there is no row exchange
            if (n.matrix[i][i] == 0)
                return 0;
            else
                ++numberOfExchanges;
        }

        for (int j = i + 1; j < n.row; ++j)
        {
            double factor = n.matrix[j][i] / n.matrix[i][i];
            for (int col_j = 0; col_j < n.col; ++col_j)
            {
                n.matrix[j][col_j] -= factor * n.matrix[i][col_j];
            }
        }
    }

    double re = 1;
    for (int i = 0; i < n.row; ++i)
    {
        re *= n.matrix[i][i];
    }

    return numberOfExchanges & 1 ? -re : re;
}

mx::Matrix mx::Matrix::trans() const
{
    double **matrix = allocate(col, row);

    for (int i = 0; i < col; ++i)
    {
        for (int j = 0; j < row; ++j)
        {
            matrix[i][j] = this->matrix[j][i];
        }
    }

    return Matrix(col, row, matrix);
}

mx::Matrix mx::Matrix::map(double (*f)(double)) const
{
    double **matrix = allocate(row, col);

    for (int i = 0; i < row; ++i)
    {
        for (int j = 0; j < col; ++j)
        {
            matrix[i][j] = f(this->matrix[i][j]);
        }
    }

    return Matrix(row, col, matrix);
}

mx::Matrix mx::Matrix::sum(int axis) const
{
    if (axis == 1)
    {
        double **matrix = allocate(1, col);

        for (int j = 0; j < col; ++j)
        {
            matrix[0][j] = 0;
            for (int i = 0; i < row; ++i)
            {
                matrix[0][j] += this->matrix[i][j];
            }
        }

        return Matrix(1, col, matrix);
    }
    else if (axis == 2)
    {
        double **matrix = allocate(row, 1);

        for (int i = 0; i < row; ++i)
        {
            matrix[i][0] = 0;
            for (int j = 0; j < col; ++j)
            {
                matrix[i][0] += this->matrix[i][j];
            }
        }

        return Matrix(row, 1, matrix);
    }

    throw std::invalid_argument("axis must be 1 or 2");
}

mx::Matrix mx::Matrix::strassen(const Matrix &l, const Matrix &r)
{
    if (l.col != r.row)
        throw std::invalid_argument("Nonconformant arguments: Matrix l and Matrix r cannot be multiplied.");

    if(l.size(1) == 1 && l.size(2) == 1)
        return l * r;

    Tuple ls = l.size(), rs = r.size();
    int padding_xl = pow(2, ceil(log2(ls.row)));
    int padding_yl = pow(2, ceil(log2(ls.col)));
    int padding_xr = pow(2, ceil(log2(rs.row)));
    int padding_yr = pow(2, ceil(log2(rs.col)));

    // Padding the matrix with 0 to make it square
    Matrix z1 = zeros(padding_xl, padding_yl);
    Matrix z2 = zeros(padding_xr, padding_yr);
    for(int i = 0; i < ls.row; ++i){
        for(int j = 0; j < ls.col;++j){
            z1.matrix[i][j] = l.matrix[i][j];
        }
    }
    for(int i = 0; i < rs.row; ++i){
        for(int j = 0; j < rs.col; ++j){
            z2.matrix[i][j] = r.matrix[i][j];
        }
    }

    Matrix a = z1(0, z1.row / 2 - 1, 0, z1.col / 2 - 1);
    Matrix b = z1(0, z1.row / 2 - 1, z1.col / 2, z1.col - 1);
    Matrix c = z1(z1.row / 2, z1.row - 1, 0, z1.col / 2 - 1);
    Matrix d = z1(z1.row / 2, z1.row - 1, z1.col / 2, z1.col - 1);

    Matrix e = z2(0, z2.row / 2 -1, 0, z2.col / 2 - 1);
    Matrix f = z2(0, z2.row / 2 - 1, z2.col / 2, z2.col - 1);
    Matrix g = z2(z2.row / 2, z2.row - 1, 0, z2.col / 2 - 1);
    Matrix h = z2(z2.row / 2, z2.row - 1, z2.col / 2, z2.col - 1);

    Matrix p1 = strassen(a, f - h);
    Matrix p2 = strassen(a + b, h);
    Matrix p3 = strassen(c + d, e);
    Matrix p4 = strassen(d, g - e);
    Matrix p5 = strassen(a + d, e + h);
    Matrix p6 = strassen(b - d, g + h);
    Matrix p7 = strassen(a - c, e + f);

    // Be aware of the destructor. Store each in different variables.
    Matrix r1t = append(p4 + p5 + p6 - p2, p1+p2, 2);
    Matrix r2t = append(p3+p4, p1+p5-p3-p7, 2);
    Matrix result = append(r1t, r2t, 1);

    double** matrix = allocate(l.row, r.col);

    for (int i = 0; i < l.row; ++i){
        for (int j = 0; j < r.col; ++j){
            matrix[i][j] = result.matrix[i][j];
        }
    }

    return Matrix(l.row, r.col, matrix);
}

mx::Matrix mx::Matrix::append(const mx::Matrix &m1, const mx::Matrix &m2, int axis)
{
    if (axis == 1)
    {
        if (m1.col != m2.col)
            throw std::invalid_argument("Matrix m2 does not have the same number of cols as Matrix m1");

        double **matrix = Matrix::allocate(m1.row + m2.row, m1.col);

        for (int i = 0; i < m1.row + m2.row; i++)
        {
            for (int j = 0; j < m1.col; ++j)
            {
                if (i >= m1.row)
                {
                    matrix[i][j] = m2.matrix[i - m1.row][j];
                }
                else
                    matrix[i][j] = m1.matrix[i][j];
            }
        }

        return Matrix(m1.row + m2.row, m1.col, matrix);
    }
    else if (axis == 2)
    {
        if (m1.row != m2.row)
            throw std::invalid_argument("Matrix m2 does not have the same number of rows as Matrix m1");

        double **matrix = Matrix::allocate(m1.row, m1.col + m2.col);

        for (int i = 0; i < m1.row; i++)
        {
            for (int j = 0; j < m1.col + m2.col; ++j)
            {
                if (j >= m1.col)
                {
                    matrix[i][j] = m2.matrix[i][j - m1.col];
                }
                else
                    matrix[i][j] = m1.matrix[i][j];
            }
        }

        return Matrix(m1.row, m1.col + m2.col, matrix);
    }

    throw std::invalid_argument("axis must be 1 or 2.");
}

void mx::Matrix::display() const
{
    for (int i = 0; i < row; ++i)
    {
        for (int j = 0; j < col; ++j)
        {
            if (j != 0)
                printf(" ");
            printf("%.8f", matrix[i][j]);
        }
        printf("\n");
    }
}

mx::Matrix mx::operator^(const Matrix &l, double expo)
{
    double **matrix = Matrix::allocate(l.row, l.col);

    for (int i = 0; i < l.row; ++i)
    {
        for (int j = 0; j < l.col; ++j)
        {
            matrix[i][j] = pow(l.matrix[i][j], expo);
        }
    }

    return Matrix(l.row, l.col, matrix);
}

mx::Matrix mx::operator+(const Matrix &l, const Matrix &r)
{
    if (l.row != r.row || l.col != r.col)
        throw std::invalid_argument("Matrix l and Matrix r must have the same size.");

    double **matrix = Matrix::allocate(l.row, l.col);

    for (int i = 0; i < l.row; ++i)
    {
        for (int j = 0; j < l.col; ++j)
        {
            matrix[i][j] = l.matrix[i][j] + r.matrix[i][j];
        }
    }

    return Matrix(l.row, l.col, matrix);
}

mx::Matrix mx::operator+(const Matrix &l, double f)
{
    return f + l;
}

mx::Matrix mx::operator+(double f, const Matrix &l)
{
    double **matrix = Matrix::allocate(l.row, l.col);

    for (int i = 0; i < l.row; ++i)
    {
        for (int j = 0; j < l.col; ++j)
        {
            matrix[i][j] = f + l.matrix[i][j];
        }
    }

    return Matrix(l.row, l.col, matrix);
}

mx::Matrix mx::operator-(const Matrix &l, const Matrix &r)
{
    if (l.row != r.row || l.col != r.col)
        throw std::invalid_argument("Matrix l and Matrix r must have the same size.");

    double **matrix = Matrix::allocate(l.row, l.col);

    for (int i = 0; i < l.row; ++i)
    {
        for (int j = 0; j < l.col; ++j)
        {
            matrix[i][j] = l.matrix[i][j] - r.matrix[i][j];
        }
    }

    return Matrix(l.row, l.col, matrix);
}

mx::Matrix mx::operator-(const Matrix &m, double f)
{
    return -f + m;
}

mx::Matrix mx::operator*(const Matrix &l, const Matrix &r)
{
    if (l.col != r.row)
        throw std::invalid_argument("Nonconformant arguments: Matrix l and Matrix r cannot be multiplied.");

    double **matrix = Matrix::allocate(l.row, r.col);

    for (int i = 0; i < l.row; ++i)
    {
        for (int j = 0; j < r.col; ++j)
        {
            matrix[i][j] = 0;
            for (int m = 0; m < l.col; ++m)
            {
                matrix[i][j] += l.matrix[i][m] * r.matrix[m][j];
            }
        }
    }

    return Matrix(l.row, r.col, matrix);
}

mx::Matrix mx::operator*(const Matrix &l, double f)
{
    return f * l;
}

mx::Matrix mx::operator*(double f, const Matrix &l)
{
    double **matrix = Matrix::allocate(l.row, l.col);

    for (int i = 0; i < l.row; ++i)
    {
        for (int j = 0; j < l.col; ++j)
        {
            matrix[i][j] = f * l.matrix[i][j];
        }
    }

    return Matrix(l.row, l.col, matrix);
}

mx::Matrix mx::operator/(const Matrix &l, double f)
{
    if (f == 0)
        throw std::invalid_argument("Divided by 0");

    return (1 / f) * l;
}

double &mx::Matrix::operator()(int r, int c)
{
    if (r < 0 || r >= this->row)
        throw std::invalid_argument("r must be greater than zero and smaller than the row");
    else if (c < 0 || c >= this->col)
        throw std::invalid_argument("c must be greater than zero and smaller than the col");

    return matrix[r][c];
}

const double &mx::Matrix::operator()(int r, int c) const
{
    if (r < 0 || r >= this->row)
        throw std::invalid_argument("r must be greater than zero and smaller than the row");
    else if (c < 0 || c >= this->col)
        throw std::invalid_argument("c must be greater than zero and smaller than the col");

    return matrix[r][c];
}

mx::Matrix mx::Matrix::operator()(int row_low, int row_high, int col_low, int col_high) const
{
    if (row_low < 0 || row_low > row_high)
        throw std::invalid_argument("row_low must be greater than 0 and smaller than or equal to row_high");
    else if (row_high >= row || row_high < row_low)
        throw std::invalid_argument("row_high must be smaller than row and greater than or equal to row_low");
    else if (col_low < 0 || col_low > col_high)
        throw std::invalid_argument("col_low must be greater than 0 and smaller than or equal to col_high");
    else if (col_high >= col || col_high < col_low)
        throw std::invalid_argument("col_high must be smaller than col and greater than or equal to col_low");

    double **matrix = Matrix::allocate(row_high - row_low + 1, col_high - col_low + 1);

    for (int i = 0; i <= row_high - row_low; ++i)
    {
        for (int j = 0; j <= col_high - col_low; ++j)
        {
            matrix[i][j] = this->matrix[row_low + i][col_low + j];
        }
    }

    return Matrix(row_high - row_low + 1, col_high - col_low + 1, matrix);
}

bool mx::operator==(const Matrix &l, const Matrix &r)
{
    if (l.row != r.row || l.col != r.col)
        return false;

    for (int i = 0; i < l.row; ++i)
    {
        for (int j = 0; j < l.col; ++j)
        {
            if (l.matrix[i][j] != r.matrix[i][j])
                return false;
        }
    }

    return true;
}

mx::Matrix mx::operator==(const Matrix &l, double d)
{
    double **matrix = Matrix::allocate(l.row, l.col);

    for (int i = 0; i < l.row; ++i)
    {
        for (int j = 0; j < l.col; ++j)
        {
            matrix[i][j] = l.matrix[i][j] == d;
        }
    }

    return Matrix(l.row, l.col, matrix);
}

mx::Matrix mx::operator<(const Matrix &l, double d)
{
    double **matrix = Matrix::allocate(l.row, l.col);

    for (int i = 0; i < l.row; ++i)
    {
        for (int j = 0; j < l.col; ++j)
        {
            matrix[i][j] = l.matrix[i][j] < d;
        }
    }

    return Matrix(l.row, l.col, matrix);
}

mx::Matrix mx::operator>(const Matrix &l, double d)
{
    double **matrix = Matrix::allocate(l.row, l.col);

    for (int i = 0; i < l.row; ++i)
    {
        for (int j = 0; j < l.col; ++j)
        {
            matrix[i][j] = l.matrix[i][j] > d;
        }
    }

    return Matrix(l.row, l.col, matrix);
}

mx::Matrix mx::operator<=(const Matrix &l, double d)
{
    double **matrix = Matrix::allocate(l.row, l.col);

    for (int i = 0; i < l.row; ++i)
    {
        for (int j = 0; j < l.col; ++j)
        {
            matrix[i][j] = l.matrix[i][j] <= d;
        }
    }

    return Matrix(l.row, l.col, matrix);
}

mx::Matrix mx::operator>=(const Matrix &l, double d)
{
    double **matrix = Matrix::allocate(l.row, l.col);

    for (int i = 0; i < l.row; ++i)
    {
        for (int j = 0; j < l.col; ++j)
        {
            matrix[i][j] = l.matrix[i][j] >= d;
        }
    }

    return Matrix(l.row, l.col, matrix);
}

mx::Matrix mx::ones(int row, int col)
{
    double **matrix = Matrix::allocate(row, col);

    for (int i = 0; i < row; ++i)
    {
        for (int j = 0; j < col; ++j)
        {
            matrix[i][j] = 1;
        }
    }

    return Matrix(row, col, matrix);
}

mx::Matrix mx::zeros(int row, int col)
{
    double **matrix = Matrix::allocate(row, col);

    for (int i = 0; i < row; ++i)
    {
        for (int j = 0; j < col; ++j)
        {
            matrix[i][j] = 0;
        }
    }

    return Matrix(row, col, matrix);
}

mx::Matrix mx::randn(int row, int col)
{
    double **matrix = Matrix::allocate(row, col);

    for (int i = 0; i < row; ++i)
    {
        for (int j = 0; j < col; ++j)
        {
            matrix[i][j] = gen();
        }
    }

    return Matrix(row, col, matrix);
}

mx::Matrix mx::eye(int size)
{
    double **matrix = Matrix::allocate(size, size);

    for (int i = 0; i < size; ++i)
    {
        for (int j = 0; j < size; ++j)
        {
            matrix[i][j] = i == j;
        }
    }

    return Matrix(size, size, matrix);
}