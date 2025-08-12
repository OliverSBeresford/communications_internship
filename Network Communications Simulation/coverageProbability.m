function coverageProbability(data, simulations)
    figure()
    
    % Names for the graph
    name = "2D with NLOS and Diffraction";
    
    % Number of bins for the histogram
    numBins = 1000;
    
    % This is where all the SINR results are stored
    results = zeros(simulations, 1);
    
    hold on
    
    for ii = 1:simulations
        % Pick a random street or avenue
        index = randperm(length(data.avenues) + length(data.streets), 1);
        
        % If it's an avenue, place the user on it at random
        if index <= length(data.avenues)
            data.receiver = [data.avenues(index), rand() * data.size - data.size / 2];
        % Do the same if it's a street
        else
            data.receiver = [rand() * data.size - data.size / 2, data.streets(index - length(data.avenues))];
        end
        
        result = SINR(data);
        results(ii) = 10 * log10(result);
    end

    % Plot the CCDF graph for this K value
    [x, y] = CCDF(results(:, 1), numBins);

    % Plot the graph
    plot(x, y, Color="r", DisplayName=name);
    
    % Label the graph
    title('Coverage probability CCDF');
    xlabel('\theta');
    ylabel('Probability');
    legend();
    
    hold off
end