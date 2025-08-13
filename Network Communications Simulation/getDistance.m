function stationDistance = getDistance(receiver, transmitter)
    % Calculates the distance between two points in (x, y) format
    stationDistance = sqrt((receiver(1) - transmitter.x)^2 + (receiver(2) - transmitter.y)^2);
end