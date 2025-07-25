clear; clc

% Create a times table with the specified height and width
width = 8;
height = 20;
timesTable = zeros(height, width);

% Add numbers to the times table
for num = 1:height
    for ii = 1:width
        timesTable(num, ii) = num * ii;
    end
end

disp(timesTable);
