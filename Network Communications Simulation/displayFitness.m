function displayFitness(data, computationNodes)
    % Lower and upper bounds for the SINR colors (in dB)
    lowerBound = -5;
    upperBound = 20;

    n_colors = upperBound - lowerBound;

    % Colors for the beginning and end of the gradient
    colorMap = hot(n_colors);

    % Colors for the map that is just red / green (normalized)
    red = [255/255, 127/255, 127/255];
    green = [209/255, 255/255, 189/255];
    redAndGreen = [red; green];
    
    % How thick the points are
    chonkiness = 500;

    % Making sure stationCount has been set
    data.stationCount = size(data.baseStations, 1);
    
    % Arrays to store the points
    heatMapPoints = zeros(computationNodes * (length(data.avenues) + length(data.streets)), 3);
    colorMapPoints = zeros(computationNodes * (length(data.avenues) + length(data.streets)), 3);

    index = 1;
    
    % Going through each avenue and looking at 500 points
    for ave = data.avenues
        for y = linspace(-data.size/2, data.size/2, computationNodes)
            % Checking SINR for a user at this point
            data.receiver = [ave, y];
            sinr = 10 * log10(SINR(data));

            % Determining color based on SINR strength
            colorIndex = clip(round(sinr), lowerBound, upperBound - 1) - lowerBound + 1;

            % Red (1) if it's below threshold, green otherwise
            color2 = 1 + (sinr > 10);
            
            % Save the points and sinr
            heatMapPoints(index, 1) = ave;
            heatMapPoints(index, 2) = y;
            heatMapPoints(index, 3) = colorIndex;
            colorMapPoints(index, 1) = ave;
            colorMapPoints(index, 2) = y;
            colorMapPoints(index, 3) = color2;

            index = index + 1;
        end
    end

    % Going through each street and looking at 500 points
    for st = data.streets
        for x = linspace(-data.size/2, data.size/2, computationNodes)
            % Checking SINR for a user at this point
            data.receiver = [x, st];
            sinr = 10 * log10(SINR(data));

            % Determining color based on SINR strength
            colorIndex = clip(round(sinr), lowerBound, upperBound - 1) - lowerBound + 1;

            % Red (1) if it's below threshold, green otherwise
            color2 = 1 + (sinr > 10);

            % Save the points and sinr
            heatMapPoints(index, 1) = x;
            heatMapPoints(index, 2) = st;
            heatMapPoints(index, 3) = colorIndex;
            colorMapPoints(index, 1) = x;
            colorMapPoints(index, 2) = st;
            colorMapPoints(index, 3) = color2;

            index = index + 1;
        end
    end
    
    %% First figure (heatmap)
    hold on
    figure(1)
    % Place all the colored markers
    scatter(heatMapPoints(:, 1), heatMapPoints(:, 2), chonkiness, colorMap(heatMapPoints(:, 3), :), Marker=".", HandleVisibility="off");
    % Draw the map
    data.drawManhattan(3 * chonkiness);

    % Label the title and axis labels
    title("Network Coverage Map");
    xlabel("x (m)");
    ylabel("y (m)");
    
    % Adding the color bar to explain color gradient
    colormap(colorMap);
    clim([lowerBound upperBound]);
    colorBar = colorbar;
    colorBar.Label.String = "SINR Value";
    
    hold off
    %% Second figure (color map)
    hold on

    figure(2)
    scatter(colorMapPoints(:, 1), colorMapPoints(:, 2), chonkiness, redAndGreen(colorMapPoints(:, 3), :), Marker=".", HandleVisibility="off");
    data.drawManhattan(3 * chonkiness);

    % Label the title and axis labels
    title("Network Coverage Map");
    xlabel("x (m)");
    ylabel("y (m)");

    hold off
end