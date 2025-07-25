function result = fibonacciQuick(index)
    % First 2 elements are 1
    if index < 3
        result = 1;
        return
    end

    % 3 Numbers to store current, previous, and before-previous numbers
    previous = 1; previous2 = 1; current = 1;
    
    % Update all the values
    for ii = 3:index
        current = previous + previous2;
        previous2 = previous;
        previous = current;
    end

    % Return the value at (index)
    result = current;
end
