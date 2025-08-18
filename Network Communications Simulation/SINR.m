function coverage = SINR(data)
    % Data must come from the custom class SimulationData
    arguments
        data(1, 1) {mustBeA(data, 'SimulationData')}
    end
    
    %% Returns the SINR (signal strength) for a data.receiver

    usefulPower = 0;

    % Creating variable to store current base station coordinates
    baseStation = [0 0];

    % If diffraction is used, calculate how many base stations there are on
    % avenues
    if data.diffractionOrder > 0 && isempty(data.numAveBases)
        data.numAveBases = sum(data.aveCounts);
    end
    
    totalInterference = 0;

    for ii = 1:data.stationCount
        % Updating the variable that stores the coordinates
        baseStation(1) = data.baseStations(ii, 1);
        baseStation(2) = data.baseStations(ii, 2);

        sameStreet = baseStation(1) == data.receiver(1) || baseStation(2) == data.receiver(2);

        if sameStreet
            % Add the interference from a LOS BS to the total interference
            p = powerLOS(data, baseStation);
            totalInterference = totalInterference + p;

            % Update useful power if this one is stronger
            if p > usefulPower; usefulPower = p; end
        elseif data.useNLOS
            % Add the interference from this NLOS base station
            p = powerNLOS(data, baseStation);
            totalInterference = totalInterference + p;

            % Update useful power if this one is stronger
            if p > usefulPower && data.connectToNLOS; usefulPower = p; end
        end
        if ~sameStreet && data.diffractionOrder > 0 && ii <= data.numAveBases
            % Add interference from diffraction
            p = diffractionPower(data, baseStation);
            totalInterference = totalInterference + p;

            % Update useful power if this one is stronger
            if p > usefulPower && data.connectToNLOS; usefulPower = p; end
        end
    end
    
    % Return if usefulPower is 0
    if usefulPower == 0
        coverage = 0;
        return;
    end

    % Get SINR value using the formula
    coverage = usefulPower / (data.noisePower + totalInterference - usefulPower);

    % There cannot be an SINR smaller than 0
    if coverage < 0
        disp(usefulPower);
        disp(coverage);
        disp(totalInterference);
        error("Negative SINR")
    end
end