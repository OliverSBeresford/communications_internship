function numRoads = numRoadsCrossed(receiver, transmitter, avenues, streets)
    % Returns the number of roads crossed to get from R to T
    numRoads = 0;

    % Store the coordinates of receiver and transmitter in variables
    [xReceive, yReceive, xTransmit, yTransmit] = deal(receiver(1), receiver(2), transmitter(1), transmitter(2));
    
    % Counting up all the vertical roads crossed
    for ave = avenues
        if (xReceive < ave && ave < xTransmit) || (xTransmit < ave && ave < xReceive)
            numRoads = numRoads + 1;
        end
    end

    % Counting up all the horizontal roads crossed
    for st = streets
        if (yReceive < st && st < yTransmit) || (yTransmit < st && st < yReceive)
            numRoads = numRoads + 1;
        end
    end
end