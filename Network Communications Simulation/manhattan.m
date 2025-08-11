function manhattan(data)
    % Homogeneous Poisson-Point Processes
    numAvenues = poissrnd(data.size * data.lambdaAve);
    numStreets = poissrnd(data.size * data.lambdaSt);
    
    % Uniformly distribute the avenues and streets
    data.avenues = rand(1, numAvenues) .* data.size - (data.size / 2);
    data.streets = [0 rand(1, numStreets) .* data.size - (data.size / 2)];
    
    % Only create base stations if requested
    if ~data.createBaseStations
        return;
    end

    % Number of base stations on each avenue, then on each street
    data.aveCounts = poissrnd(data.lambdaBase * data.size, 1, numAvenues);
    data.stCounts = poissrnd(data.lambdaBase * data.size, 1, numStreets + 1);

    % Create matrix for all the stations
    data.stationCount = sum(data.aveCounts) + sum(data.stCounts);
    data.baseStations = zeros(data.stationCount, 2);

    index = 1;

    % Creating base stations
    for ii = 1:numAvenues
        % Row 1 is avenues, column is the base station count for that ave
        thisAveCount = data.aveCounts(ii);

        % Calculating #stations and creating matrix with (x, y) coordinates
        stations = zeros(thisAveCount, 2);

        % Their y coordinate is random and uniformly distrubuted
        stations(:, 2) = rand(1, thisAveCount) .* data.size - data.size / 2;
        % Their x coordinates are all equal to the avenue's
        stations(:, 1) = data.avenues(ii);

        % Updating main matrix
        data.baseStations(index:index + thisAveCount - 1, :) = stations;
        index = index + thisAveCount;
    end
    for ii = 1:numStreets + 1
        % Row 2 is streets, column is the base station count for that st
        thisStCount = data.stCounts(ii);

        % Calculating #stations and creating matrix with (x, y) coordinates
        stations = zeros(thisStCount, 2);

        % Their x coordinates are random and uniformly distrubuted
        stations(:, 1) = rand(1, thisStCount) .* data.size - data.size / 2;
        % Their y coordinates are all equal to the avenue's
        stations(:, 2) = data.streets(ii);

        % Updating main matrix
        data.baseStations(index:index + thisStCount - 1, :) = stations;
        index = index + thisStCount;
    end
end