#ifndef my_matrix
#define my_matrix

namespace mx
{
    struct Tuple
    {
        int row, col;
    };

    class Matrix
    {
    private:
        int row, col;
        double **matrix;

        // delta for pseudo-inverse
        constexpr static double delta = 0.0001;

    private:
        /**
         * @brief Allocate row*col space for a double** pointer
         *
         * @param row
         * @param col
         * @return double**
         */
        static double **allocate(int row, int col);

        /**
         * @brief Free the allocated space for the double** matrix
         *
         */
        void freeResources();

        /**
         * @brief Construct a new Matrix object by using existing double** ptr
         *
         * @param row
         * @param col
         * @param matrix
         */
        Matrix(int row, int col, double **matrix);

        /**
         * @brief Exchange the row of the matrix. It will be used in finding inverse and determinant
         *
         * @param r1 row 1
         * @param r2 row 2
         */
        void exchangeRow(int r1, int r2);

    public:

        /**
         * @brief Construct a new Matrix object
         *
         * @param row
         * @param col
         */
        Matrix(int row, int col);

        /**
         * @brief Construct a new Matrix object by deepcopying another Matrix object
         *
         * @param m
         */
        Matrix(const Matrix &m);
        ~Matrix();

        /**
         * @brief Append two matrices by the specified axis
         *
         * @param m1 Matrix 1
         * @param m2 Matrix 2
         * @param axis 1 for row, 2 for column
         * @return Matrix
         */
        static Matrix append(const Matrix &m1, const Matrix &m2, int axis);

        /**
         * @brief Multiply two matrices by the strassen algorithm
         *
         * @param l A Matrix object
         * @param r A Matrix object
         * @return Matrix
         *
         * @deprecated The strassen algorithm is not used in the * multiplication.
         */
        static Matrix strassen(const Matrix& l, const Matrix& r);


        /**
         * @brief Return the size of the matrix
         *
         * @return Tuple - A Tuple containing the row and col of the matrix
         */
        Tuple size() const;

        /**
         * @brief Return the size of the matrix in specified axis
         *
         * @param axis 1 for row, 2 for column
         * @return int
         */
        int size(int axis) const;

        /**
         * @brief Return the inverse of the matrix
         *
         * @return Matrix
         */
        Matrix inv() const;

        /**
         * @brief Return the Moore-Penrose Pseudoinverse of the matrix
         *
         * @return Matrix
         */
        Matrix pinv() const;

        /**
         * @brief Return the determinant of the matrix
         *
         * @return double
         */
        double det() const;

        /**
         * @brief Return the transpose of the matrix
         *
         * @return Matrix
         */
        Matrix trans() const;

        /**
         * @brief Map a function to the elements of the matrix
         *
         * @param f a `double (*f)(double)` function
         * @return Matrix
         */
        Matrix map(double (*f)(double)) const;

        /**
         * @brief Return the sum of the matrix, specified by the axis
         *
         * @param axis 1 for row, 2 for column
         * @return Matrix
         */
        Matrix sum(int axis) const;

        /**
         * @brief Display the matrix to stdout
         *
         */
        void display() const;

        /**
         * @brief Overloaded () for accessing and changing the element of the matrix
         *
         * @param r row
         * @param c column
         * @return double&
         */
        double &operator()(int r, int c);
        const double &operator()(int r, int c) const;

        /**
         * @brief Overloaded () to support the slice of the matrix
         *
         * @param row_low lower index of the row (row-start)
         * @param row_high higher index of the row (row-end)
         * @param col_low lower index of the column (column-start)
         * @param col_high higher index of the column (column-end)
         * @return Matrix
         */
        Matrix operator()(int row_low, int row_high, int col_low, int col_high) const;

        friend Matrix operator^(const Matrix &l, double expo);

        // Overload + for scalar and matrix addition
        friend Matrix operator+(const Matrix &l, const Matrix &r);
        friend Matrix operator+(const Matrix &l, double f);
        friend Matrix operator+(double f, const Matrix &l);

        // Overload - for scalar and matrix substraction
        friend Matrix operator-(const Matrix &l, const Matrix &r);
        friend Matrix operator-(const Matrix &l, double f);

        // Overload * for scalar and matrix operations
        friend Matrix operator*(const Matrix &l, const Matrix &r);
        friend Matrix operator*(const Matrix &l, double f);
        friend Matrix operator*(double f, const Matrix &l);

        // Overload / for scalar division
        friend Matrix operator/(const Matrix &l, double f);

        // Check if two matrixes are equal
        friend bool operator==(const Matrix &l, const Matrix &r);

        friend Matrix operator==(const Matrix &l, double d);
        friend Matrix operator<(const Matrix &l, double d);
        friend Matrix operator>(const Matrix &l, double d);
        friend Matrix operator<=(const Matrix &l, double d);
        friend Matrix operator>=(const Matrix &l, double d);

        friend Matrix ones(int row, int col);
        friend Matrix zeros(int row, int col);
        friend Matrix randn(int row, int col);
        friend Matrix eye(int size);
    };

    /**
     * @brief Raise every element of the matrix to the power of expo
     *
     * @param l A matrix object
     * @param expo Exponential
     * @return Matrix
     */
    mx::Matrix operator^(const Matrix &l, double expo);

    /**
     * @brief Matrix addition
     *
     * @param l A Matrix object
     * @param r A Matrix object
     * @return Matrix
     */
    mx::Matrix operator+(const Matrix &l, const Matrix &r);

    /**
     * @brief Add every element of the matrix by f
     *
     * @param l A Matrix object
     * @param f A double value
     * @return Matrix
     */
    mx::Matrix operator+(const Matrix &l, double f);
    mx::Matrix operator+(double f, const Matrix &l);

    /**
     * @brief  Matrix substraction
     *
     * @param l A Matrix object
     * @param r A Matrix object
     * @return Matrix
     */
    mx::Matrix operator-(const Matrix &l, const Matrix &r);

    /**
     * @brief Subtract every element of the matrix by f
     *
     * @param l A Matrix object
     * @param f A double value
     * @return Matrix
     */
    mx::Matrix operator-(const Matrix &l, double f);

    /**
     * @brief Multiply two matrices
     *
     * @param l A Matrix object
     * @param r A Matrix object
     * @return Matrix
     */
    mx::Matrix operator*(const Matrix &l, const Matrix &r);

    /**
     * @brief Multiply every element of the matrix by f
     *
     * @param l A Matrix object
     * @param f A double value
     * @return Matrix
     */
    mx::Matrix operator*(const Matrix &l, double f);
    mx::Matrix operator*(double f, const Matrix &l);

    /**
     * @brief Divide every element of the matrix by f
     *
     * @param l A Matrix object
     * @param f A double value
     * @return Matrix
     */
    mx::Matrix operator/(const Matrix &l, double f);

    /**
     * @brief Compare if two matrixes are equal (have the same dimensions and same elements)
     *
     * @param l A Matrix object
     * @param r A Matrix object
     * @return bool
     */
    bool operator==(const Matrix &l, const Matrix &r);

    /**
     * @brief Performing == d on every element of the matrix
     *
     * @param l A Matrix object
     * @param d A double value
     * @return Matrix
     */
    mx::Matrix operator==(const Matrix &l, double d);

    /**
     * @brief Performing < d on every element of the matrix
     *
     * @param l A Matrix object
     * @param d A double value
     * @return Matrix
     */
    mx::Matrix operator<(const Matrix &l, double d);

    /**
     * @brief Performing > d on every element of the matrix
     *
     * @param l A Matrix object
     * @param d A double value
     * @return Matrix
     */
    mx::Matrix operator>(const Matrix &l, double d);

    /**
     * @brief Performing <= d on every element of the matrix
     *
     * @param l A Matrix object
     * @param d A double value
     * @return Matrix
     */
    mx::Matrix operator<=(const Matrix &l, double d);

    /**
     * @brief Performing >= d on every element of the matrix
     *
     * @param l A Matrix object
     * @param d A double value
     * @return Matrix
     */
    mx::Matrix operator>=(const Matrix &l, double d);

    /**
     * @brief Generate a row*col ones matrix
     *
     * @param row
     * @param col
     * @return Matrix
     */
    Matrix ones(int row, int col);

    /**
     * @brief Generate a row*col zeros matrix
     *
     * @param row
     * @param col
     * @return Matrix
     */
    Matrix zeros(int row, int col);

    /**
     * @brief Generate a row*col random matrix
     *
     * @param row
     * @param col
     * @return Matrix
     */
    Matrix randn(int row, int col);

    /**
     * @brief Generate a size*size identity matrix
     *
     * @param size
     * @return Matrix
     */
    Matrix eye(int size);
}

#endif