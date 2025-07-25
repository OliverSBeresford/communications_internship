function result = fibonacciButBad(index)
    if index == 1 || index == 2
        result = 1;
        return
    end

    result = fibonacciButBad(index - 1) + fibonacciButBad(index - 2);
