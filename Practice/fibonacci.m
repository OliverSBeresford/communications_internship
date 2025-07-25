function result = fibonacci(index)
    % First 2 elements are 1
    if index < 3
        result = 1;
        return
    end

    % Sequence that will contain the fibanacci sequence
    sequence = ones(index);
    
    % Update all the values
    for ii = 3:index
        sequence(ii) = sequence(ii - 1) + sequence(ii - 2);
    end

    % Return the value at (index)
    result = sequence(index);
end
