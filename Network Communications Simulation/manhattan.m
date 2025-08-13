function manhattan(data)
    % Homogeneous Poisson-Point Processes
    numAvenues = poissrnd(data.size * data.lambdaAve);
    numStreets = poissrnd(data.size * data.lambdaSt - 1);
    
    % Uniformly distribute the avenues and streets
    data.avenues = rand(1, numAvenues) .* data.size - (data.size / 2);
    data.streets = [0 rand(1, numStreets) .* data.size - (data.size / 2)];

    % Increment street count by 1 for the added street
    numStreets = numStreets + 1;
    
    % Only create base stations if requested
    if ~data.createBaseStations
        return;
    end

    % Number of base stations on each avenue, then on each street
    data.aveCounts = poissrnd(data.lambdaBase * data.size, 1, numAvenues);
    data.stCounts = poissrnd(data.lambdaBase * data.size, 1, numStreets);

    % Create matrix for all the stations
    data.stationCount = sum(data.aveCounts) + sum(data.stCounts);
    data.baseStations = createArray(data.stationCount, 1, "BaseStation");

    index = 1;

    % Creating base stations
    for ii = 1:numAvenues
        % Row 1 is avenues, column is the base station count for that ave
        thisAveCount = data.aveCounts(ii);

        % Their y coordinate is random and uniformly distrubuted
        y = rand(1, thisAveCount) .* data.size - data.size / 2;
        % Their x coordinates are all equal to the avenue's
        x = data.avenues(ii);

        % Updating main matrix
        for j = 1:thisAveCount
            data.baseStations(index).y = y(j);
            data.baseStations(index).x = x;
            index = index + 1;
        end
    end
    for ii = 1:numStreets
        % Row 2 is streets, column is the base station count for that st
        thisStCount = data.stCounts(ii);

        % Their x coordinates are random and uniformly distrubuted
        x = rand(1, thisStCount) .* data.size - data.size / 2;
        % Their y coordinates are all equal to the avenue's
        y = data.streets(ii);

        % Updating main matrix
        for j = 1:thisStCount
            data.baseStations(index).y = y;
            data.baseStations(index).x = x(j);
            index = index + 1;
        end
    end
end