function displayFitness(data, computationNodes)
    % Making sure stationCount has been set
    data.stationCount = size(data.baseStations, 1);

    hold on
    
    % Going through each avenue and looking at 500 points
    for ave = data.avenues
        for y = linspace(-data.size/2, data.size/2, computationNodes)
            % Checking SINR for a user at this point
            data.receiver = [ave, y];
            sinr = 10 * log10(SINR(data));

            % Indicator function: increase fitness if SINR > threshold
            if sinr > data.thresholdDB
                plot(ave, y, Marker=".", Color="r", HandleVisibility="off");
            end
        end
    end

    % Going through each street and looking at 500 points
    for st = data.streets
        for x = linspace(-data.size/2, data.size/2, computationNodes)
            % Checking SINR for a user at this point
            data.receiver = [x, st];
            sinr = 10 * log10(SINR(data));

            % Indicator function: increase fitness if SINR > threshold
            if sinr > data.thresholdDB
                plot(x, st, Marker=".", Color="r", HandleVisibility="off");
            end
        end
    end

    hold off
end