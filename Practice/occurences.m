% Counts the number of occurences of (number) in (vector)
function num = occurences(vector, number)
    num = 0;

    for ii = vector
        if ii == number
            num = num + 1;
        end
    end
end
