function sortedVector = bubbleSort(vector, ascending)
    len = length(vector);
    notSorted = true;
    
    % Default value of ascending is true
    if nargin < 2
        ascending = true;
    end
    
    % Keep bubbling up values until the vector is sorted
    while notSorted
        switchCount = 0;
        
        % Look at every pair of numbers in the vector
        for ii = 2:len
            % If 2 elements are in the wrong order, switch them
            if ascending && vector(ii) < vector (ii - 1)
                [vector(ii), vector(ii - 1)] = deal(vector(ii - 1), vector(ii));
                switchCount = switchCount + 1;
            elseif ~ascending && vector(ii) > vector(ii - 1)
                [vector(ii), vector(ii - 1)] = deal(vector(ii - 1), vector(ii));
                switchCount = switchCount + 1;
            end
        end

        % If you made no changes, the vector is sorted, so stop
        if switchCount == 0
            notSorted = false;
        end
    end
    
    % Return the sorted vector
    sortedVector = vector;

end
