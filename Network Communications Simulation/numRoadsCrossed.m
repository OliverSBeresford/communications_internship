function numRoads = numRoadsCrossed(data, transmitter)
    % Returns the number of roads crossed to get from R to T
    numRoads = 0;

    % Store the coordinates of data.receiver and transmitter in variables
    xReceive = data.receiver(1);
    yReceive = data.receiver(2);
    xTransmit = transmitter(1);
    yTransmit = transmitter(2);
    
    % Counting up all the vertical roads crossed
    for ave = data.avenues
        if (xReceive < ave && ave < xTransmit) || (xTransmit < ave && ave < xReceive)
            numRoads = numRoads + 1;
        end
    end

    % Counting up all the horizontal roads crossed
    for st = data.streets
        if (yReceive < st && st < yTransmit) || (yTransmit < st && st < yReceive)
            numRoads = numRoads + 1;
        end
    end
end