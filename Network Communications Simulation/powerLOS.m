function power = powerLOS(data, transmitter)
    % Calculates power received from an LOS base station from the formula
    distance = getDistance(data.receiver, transmitter);
    pathLoss = data.A * distance ^ (-data.alpha);
    power = data.sourcePower * smallScaleFading(data.fadingMean) * pathLoss;
end