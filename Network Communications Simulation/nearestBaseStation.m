function [station, closestIndex] = nearestBaseStation(data)
    % A vector with the distance from data.receiver to every base station
    distances = sqrt(...
        (data.baseStations(:, 1) - data.receiver(1)) .^ 2 + ...
        (data.baseStations(:, 2) - data.receiver(2)) .^ 2);
    
    % Get the index of the closest base station
    [~, closestIndex] = min(distances);
    
    % Coordinates of the base station
    station = data.baseStations(closestIndex);
end