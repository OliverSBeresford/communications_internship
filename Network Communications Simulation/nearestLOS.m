function [station, closestIndex] = nearestLOS(receiver, baseStations, stationCount)
    % A vector with the distance from receiver to every base station
    distances = sqrt(...
        (baseStations(:, 1) - receiver(1)) .^ 2 + ...
        (baseStations(:, 2) - receiver(2)) .^ 2);

    closestIndex = -1;
    closestDistance = -1;
    
    for ii = 1:stationCount
        baseStation = baseStations(ii, :);

        % Checks if the base station is on the same street as the user
        sameStreet = baseStation(1) == receiver(1) || baseStation(2) == receiver(2);

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
        station = baseStations(closestIndex, :);
    end
end