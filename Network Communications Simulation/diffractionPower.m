function power = diffractionPower(data, baseStation)
    arguments
        data(1, 1) {mustBeA(data, 'SimulationData')}
        baseStation {mustBeVector}
    end

    power = 0;
    
    % If they are not using diffraction, return
    if data.diffractionOrder <= 0
        power = 0;
        return;
    end
    
    %% Implementing first-order diffraction
    % Real distance along two paths to get to the receiver
    dist0 = abs(data.receiver(2) - baseStation.y);
    dist1 = abs(data.receiver(1) - baseStation.x);
    
    % Fictitious distance accounting for diffraction loss
    fakeDist = dist0 + dist1 + data.q90 * dist0 * dist1;

    power = power + data.A * fakeDist ^ (-data.alpha);

    % Power cannot be greater than transmitted power
    if power > baseStation.power
        power = baseStation.power;
    end
end