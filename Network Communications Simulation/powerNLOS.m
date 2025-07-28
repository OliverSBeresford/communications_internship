function power = powerNLOS(sourcePower, receiver, transmitter, fadingMean, penetrationLoss, avenues, streets)
    % Number of roads crossed = number of buildings penetrated
    buildings = 1 + numRoadsCrossed(receiver, transmitter, avenues, streets);
    
    power = sourcePower * smallScaleFading(fadingMean) * penetrationLoss ^ buildings;
end