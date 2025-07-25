clear; clc

% Empty vector
multiples = zeros(1, 10);
multiple = 16;
number = 14;

% Find all multiples of multiple from 1 to number
for ii = 1:number
    multiples(ii) = ii .* multiple;
end

disp(multiples)