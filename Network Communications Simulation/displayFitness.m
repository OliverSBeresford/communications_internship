function displayFitness(data, computationNodes)
    % Lower and upper bounds for the SINR colors (in dB)
    lowerBound = -5;
    upperBound = 20;

    n_colors = upperBound - lowerBound;

    % Colors for the beginning and end of the gradient
    navy = [0 0 0.5];
    yellow = [1 1 0];
    
    % Vectors with all the needed colors in order of weakest to strongest
    % SINR
    R = linspace(navy(1), yellow(1), n_colors);
    G = linspace(navy(2), yellow(2), n_colors);
    B = linspace(navy(3), yellow(3), n_colors);
    
    % How thick the points are
    chonkiness = 20;

    % Making sure stationCount has been set
    data.stationCount = size(data.baseStations, 1);

    hold on
    
    % Going through each avenue and looking at 500 points
    for ave = data.avenues
        for y = linspace(-data.size/2, data.size/2, computationNodes)
            % Checking SINR for a user at this point
            data.receiver = [ave, y];
            sinr = 10 * log10(SINR(data));
    
            % Determining color based on SINR strength
            index = clip(round(sinr), lowerBound, upperBound - 1) - lowerBound + 1;
            color = [R(index) G(index) B(index)];

            % Plot the point
            plot(ave, y, Marker=".", Color=color, HandleVisibility="off", MarkerSize=chonkiness);
        end
    end

    % Going through each street and looking at 500 points
    for st = data.streets
        for x = linspace(-data.size/2, data.size/2, computationNodes)
            % Checking SINR for a user at this point
            data.receiver = [x, st];
            sinr = 10 * log10(SINR(data));

            % Determining color based on SINR strength
            index = clip(round(sinr), lowerBound, upperBound - 1) - lowerBound + 1;
            color = [R(index) G(index) B(index)];

            % Plot the point
            plot(x, st, Marker=".", Color=color, HandleVisibility="off", MarkerSize=chonkiness);
        end
    end

    data.drawManhattan(20 * chonkiness);

    hold off
end