function frequentNum = mostFrequentButBad(vector)
    max = 0;
    frequentNum = 0;

    % Make sure the vector isn't empty
    if isempty(vector)
        frequentNum = "Vector is empty";
        return
    end

    % Going through each item in the vector
    for ii = vector
        % Count the number of occurences of the number
        number = occurences(vector, ii);
        if number > max
            max = number;
            frequentNum = ii;
        end
    end
end
