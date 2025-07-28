function [x, y] = CCDF(xValues)
    % Make a histogram without plotting it
    [values, edges] = histcounts(xValues, 100, Normalization='cdf');
    
    % Plot the x values as the centers of the bins
    x = edges(1:end - 1) + (edges(2) - edges(1)) / 2;

    % Get the y values as the values of the bins
    y = 1 - values;
end