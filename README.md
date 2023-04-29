# Determinant

A simple program to calculate the determinant of a matrix.

The matrix is given as a vector of row vectors.

We use Cramer's formula (work on an implentation using Bezout's method is as of now unfinished),
and use Heap's algorithm to generate all permutations of `n` elements.

Currently works on 32 bit integer square matrices.

## TODO
make this into a webapp, on the `webapp` branch.

make the determinant return an Option or Result
instead of just an integer
