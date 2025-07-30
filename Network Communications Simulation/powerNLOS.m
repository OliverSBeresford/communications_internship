function power = powerNLOS(data, transmitter)
    % Data must come from the custom class SimulationData
    arguments
        data(1, 1) {mustBeA(data, 'SimulationData')}
        transmitter {mustBeVector}
    end

    % Number of roads crossed = number of buildings penetrated
    buildings = 1 + numRoadsCrossed(data, transmitter);
    
    % Calculate path loss
    distance = getDistance(data.receiver, transmitter);
    pathLoss = data.A * distance ^ (-data.alpha);
    
    power = data.sourcePower * smallScaleFading(data.fadingMean) * data.penetrationLoss ^ buildings * pathLoss;
end