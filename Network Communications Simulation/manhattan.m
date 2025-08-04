function [avenues, streets, allStations, stationCount, aveCounts, stCounts] = manhattan(size, lambdaBase, lambdaStreet, lambdaAvenue)
    % Homogeneous Poisson-Point Processes
    numAvenues = poissrnd(size * lambdaAvenue);
    numStreets = poissrnd(size * lambdaStreet);
    
    % Uniformly distribute the avenues and streets
    avenues = rand(1, numAvenues) .* size - (size / 2);
    streets = [0 rand(1, numStreets) .* size - (size / 2)];
    
    % Number of base stations on each avenue, then on each street
    aveCounts = poissrnd(lambdaBase * size, 1, numAvenues);
    stCounts = poissrnd(lambdaBase * size, 1, numStreets + 1);

    % Create matrix for all the stations
    stationCount = sum(aveCounts) + sum(stCounts);
    allStations = zeros(stationCount, 2);
    
    index = 1;

    % Creating base stations
    for ii = 1:numAvenues
        % Row 1 is avenues, column is the base station count for that ave
        thisAveCount = aveCounts(ii);

        % Calculating #stations and creating matrix with (x, y) coordinates
        stations = zeros(thisAveCount, 2);

        % Their y coordinate is random and uniformly distrubuted
        stations(:, 2) = rand(1, thisAveCount) .* size - size / 2;
        % Their x coordinates are all equal to the avenue's
        stations(:, 1) = avenues(ii);

        % Updating main matrix
        allStations(index:index + thisAveCount - 1, :) = stations;
        index = index + thisAveCount;
    end
    for ii = 1:numStreets + 1
        % Row 2 is streets, column is the base station count for that st
        thisStCount = stCounts(ii);

        % Calculating #stations and creating matrix with (x, y) coordinates
        stations = zeros(thisStCount, 2);

        % Their x coordinates are random and uniformly distrubuted
        stations(:, 1) = rand(1, thisStCount) .* size - size / 2;
        % Their y coordinates are all equal to the avenue's
        stations(:, 2) = streets(ii);

        % Updating main matrix
        allStations(index:index + thisStCount - 1, :) = stations;
        index = index + thisStCount;
    end
end