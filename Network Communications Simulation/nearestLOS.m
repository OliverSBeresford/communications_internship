function [station, closestIndex] = nearestLOS(data)
    % A vector with the distance from data.receiver to every base station
    distances = sqrt(...
        (data.baseStations(:, 1) - data.receiver(1)) .^ 2 + ...
        (data.baseStations(:, 2) - data.receiver(2)) .^ 2);

    closestIndex = -1;
    closestDistance = -1;

    % Creating variable to store current base station coordinates
    baseStation = [0 0];
    
    for ii = 1:data.stationCount
        % Updating the variable that stores the coordinates
        baseStation(1) = data.baseStations(ii, 1);
        baseStation(2) = data.baseStations(ii, 2);

        % Checks if the base station is on the same street as the user
        sameStreet = baseStation(1) == data.receiver(1) || baseStation(2) == data.receiver(2);

        % If it is the min distance, update to this base station
        if sameStreet && (distances(ii) <= closestDistance || closestDistance == -1)
            closestIndex = ii;
            closestDistance = distances(ii);
        end
    end
    
    % No LOS base station, so return -1, -1
    if closestDistance == -1
        station = (-1 -1);
    % Return coordinates of the nearest LOS base station
    else
        station = data.baseStations(closestIndex, :);
    end
end