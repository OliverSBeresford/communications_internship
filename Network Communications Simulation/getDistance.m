function stationDistance = getDistance(receiver, transmitter)
    % Calculates the distance between two points in (x, y) format
    stationDistance = sqrt((receiver(1) - transmitter(1))^2 + (receiver(2) - transmitter(2))^2);
end