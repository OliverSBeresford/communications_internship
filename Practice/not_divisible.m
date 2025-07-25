clear; clc

sumNotDivisible = 0;

% If a number is divisible by divisor, it will not be added to the sum
divisor = 3;
% Add all numbers up to (number)
number = 1000;

% Sum numbers 1-(number) that aren't divisible by (divisor)
for ii = 1:number
    if mod(ii, divisor) ~= 0
        sumNotDivisible = sumNotDivisible + ii;
    end
end

disp(sumNotDivisible);