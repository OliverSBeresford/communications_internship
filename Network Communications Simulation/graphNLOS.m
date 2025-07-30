simulations = 1e4;

kValues = 10.^(-1:-1:-5);
colors = [0 0 0; 0.25 0 0; 0.5 0 0; 0.75 0 0; 1 0 0];

% Number of bins for the histogram
numBins = 300;

% This is where all the SINR results are stored
results = zeros(length(kValues) + 1, simulations);

% Initialize the object that stores the data in intermediate steps
% Provide all the parameters to SimulationData
data = SimulationData( ...
    useNLOS=false, ...
    size=50, ...
    lambdaBase=0.1, ...
    lambdaAve=1, ...
    lambdaSt=1, ...
    sourcePower=1, ...
    alpha=4, ...
    A=1, ...
    fadingMean=1, ...
    noisePower=0, ...
    doManhattan=true ...
);

% Calculate SINR (simulations) times for the 1d network
for j = 1:simulations
    data.runManhattan();
    result = SINR(data);
    results(1, j) = 10 * log10(result);
end

% Plot the CCDF graph for 1d
figure(1)
hold on

[x, y] = CCDF(results(1, :), numBins);

% Plot the 1d graph
plot(x, y, Color='b');

% Plot each line for different K values
data.useNLOS = true;
for ii = 1:length(kValues)
    % Set the new K value each iteration
    data.penetrationLoss = kValues(ii);
    for j = 1:simulations
        data.runManhattan();
        result = SINR(data);
        results(ii + 1, j) = 10 * log10(result);
    end
    
    % Plot the CCDF graph for this K value
    [x, y] = CCDF(results(ii + 1, :), numBins);

    % Plot the graph
    plot(x, y, Color=colors(ii, :));
end

% Label the graph
title('Coverage probability CCDF');
xlabel('\theta');
ylabel('Probability');

hold off
