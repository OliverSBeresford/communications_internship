function power = powerLOS(data, transmitter)
    % Calculates power received from an LOS base station from the formula
    distance = getDistance(data.receiver, transmitter);
    pathLoss = data.A * distance ^ (-data.alpha);
    power = transmitter.power * smallScaleFading(data.fadingMean) * pathLoss;
    
    % Limit received power to transmitted power
    if power > transmitter.power
        power = transmitter.power;
    end
end