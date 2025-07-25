clear; clc

% Make vectors
a = [2 2 1];
b = [3 1 4];

% Multiple the vectors and display output
disp("Vector product");
c = a .* b;
disp(c);

% Make 2 simple matrices
A = [1 1; 2 0];
B = [3 1; 4 1];

% Multiply the matrices in-place and display output
disp("Matrix product");
C = A .* B;
disp(C);

