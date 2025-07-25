function power = powerLOS(sourcePower, receiver, transmitter, alpha, A, fadingMean)
    % Calculates power received from an LOS base station from the formula
    distance = getDistance(receiver, transmitter);
    pathLoss = A * distance ^ (-alpha);
    power = sourcePower * smallScaleFading(fadingMean) * pathLoss;
end