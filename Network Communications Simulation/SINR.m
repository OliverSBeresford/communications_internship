function coverage = SINR(sourcePower, receiver, alpha, A, fadingMean, noiseP, baseStations, stationCount)
    % Returns the SINR (signal strength) for a receiver
    [station, stationIndex] = nearestLOS(receiver, baseStations, stationCount);
    if stationIndex == -1
        coverage = 0;
        return;
    end
    
    % Calculates the power received from the connected base station
    usefulPower = powerLOS(sourcePower, receiver, station, alpha, A, fadingMean);
    
    totalInterference = 0;
    for ii = 1:stationCount
        baseStation = baseStations(ii, :);
        sameStreet = baseStation(1) == receiver(1) || baseStation(2) == receiver(2);
        notSource = ii ~= stationIndex;
        if sameStreet && notSource
            % Add the interference from a BS to the total interference
            totalInterference = totalInterference + powerLOS(sourcePower, receiver, baseStation, alpha, A, fadingMean);
        end
    end
    
    % Get SINR value using the formula
    coverage = usefulPower / (noiseP + totalInterference);
end