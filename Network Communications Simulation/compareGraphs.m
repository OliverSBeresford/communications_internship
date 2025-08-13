simulations = 100;

colors = [1 0 0; 0.5 0 0; 0 0 0];

% Names for the graph
names = ["1D Network", "2D with NLOS", "2D with NLOS and Diffraction"];

% Number of bins for the histogram
numBins = 1000;

% This is where all the SINR results are stored
results = zeros(3, simulations);

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
    doManhattan=true, ...
    pathLossNLOS=false, ...
    connectToNLOS=true ...
);

hold on

% Plot each line for different K values
for ii = 1:3
    % ii = 1: Just LOS (1D simulation)
    % ii = 2: LOS and NLOS interference (not including diffraction)
    % ii = 3: LOS, NLOS, and NLOS diffraction interference
    if ii == 2
        data.useNLOS = true;
        data.penetrationLoss = 0.1;
    elseif ii == 3
        data.diffractionOrder = 1;
    end

    for j = 1:simulations
        data.runManhattan();
        result = SINR(data);
        results(ii, j) = 10 * log10(result);
    end
    
    % Plot the CCDF graph for this K value
    [x, y] = CCDF(results(ii, :), numBins);

    % Plot the graph
    plot(x, y, Color=colors(ii, :), DisplayName=names(ii));
end

% Label the graph
title('Coverage probability CCDF');
xlabel('\theta');
ylabel('Probability');
legend();

hold off
