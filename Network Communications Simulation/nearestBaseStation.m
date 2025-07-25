function [station, closestIndex] = nearestBaseStation(receiver, baseStations)
    % A vector with the distance from receiver to every base station
    distances = sqrt(...
        (baseStations(:, 1) - receiver(1)) .^ 2 + ...
        (baseStations(:, 2) - receiver(2)) .^ 2);
    
    % Get the index of the closest base station
    [~, closestIndex] = min(distances);
    
    % Coordinates of the base station
    station = baseStations(closestIndex);
end