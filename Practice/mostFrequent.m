function frequentNum = mostFrequent(vector)
    max = 0;
    frequentNum = 0;

    % Make sure the vector isn't empty
    if isempty(vector)
        frequentNum = "Vector is empty";
        return
    end

    % Make a dictionary for appearances
    appearances = configureDictionary("double", "double");

    % Going through each item in the vector
    for ii = vector
        % Create the key if it doesn't already exist
        if ~isKey(appearances, ii)
            appearances(ii) = 0;
        end
        
        % Add 1 appearance of the number (ii)
        appearances(ii) = appearances(ii) + 1;
        
        % If this is the most frequent number, set max and frequentNum
        if appearances(ii) > max
            max = appearances(ii);
            frequentNum = ii;
        end
    end
end
