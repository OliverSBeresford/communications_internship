function coverage = SINR(data)
    % Data must come from the custom class SimulationData
    arguments
        data(1, 1) {mustBeA(data, 'SimulationData')}
    end
    
    % Returns the SINR (signal strength) for a data.receiver
    [station, stationIndex] = nearestLOS(data);
    if stationIndex == -1
        coverage = 0;
        return;
    end
    
    % Calculates the power received from the connected base station
    usefulPower = powerLOS(data, station);

    % Creating variable to store current base station coordinates
    baseStation = [0 0];
    
    totalInterference = 0;
    for ii = 1:data.stationCount
        % Updating the variable that stores the coordinates
        baseStation(1) = data.baseStations(ii, 1);
        baseStation(2) = data.baseStations(ii, 2);

        sameStreet = baseStation(1) == data.receiver(1) || baseStation(2) == data.receiver(2);
        notSource = ii ~= stationIndex;
        
        if sameStreet && notSource
            % Add the interference from a LOS BS to the total interference
            totalInterference = totalInterference + powerLOS(data, baseStation);
        elseif notSource && data.useNLOS
            % Add the interference from this NLOS base station
            totalInterference = totalInterference + powerNLOS(data, baseStation);
        end
    end
    
    % Get SINR value using the formula
    coverage = usefulPower / (data.noisePower + totalInterference);
end