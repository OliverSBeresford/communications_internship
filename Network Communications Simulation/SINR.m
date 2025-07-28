function coverage = SINR(data)
    % Data must come from the custom class SimulationData
    arguments
        data(1, 1) {mustBeA(data, 'SimulationData')}
    end
    
    % Returns the SINR (signal strength) for a data.receiver
    [station, stationIndex] = nearestLOS(data.receiver, data.baseStations, data.stationCount);
    if stationIndex == -1
        coverage = 0;
        return;
    end
    
    % Calculates the power received from the connected base station
    usefulPower = powerLOS(data.sourcePower, data.receiver, station, data.alpha, data.A, data.fadingMean);
    
    totalInterference = 0;
    for ii = 1:data.stationCount
        baseStation = data.baseStations(ii, :);
        sameStreet = baseStation(1) == data.receiver(1) || baseStation(2) == data.receiver(2);
        notSource = ii ~= stationIndex;
        if sameStreet && notSource
            % Add the interference from a BS to the total interference
            totalInterference = totalInterference + powerLOS(data.sourcePower, data.receiver, baseStation, data.alpha, data.A, data.fadingMean);
        elseif notSource && data.useNLOS
            % Add the interference from this NLOS base station
            totalInterference = totalInterference + powerNLOS(data, baseStation);
        end
    end
    
    % Get SINR value using the formula
    coverage = usefulPower / (data.noisePower + totalInterference);
end