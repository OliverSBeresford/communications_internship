% Initialize the object that stores the data in intermediate steps
% Provide all the parameters to SimulationData
data = SimulationData( ...
    useNLOS=true, ...
    diffractionOrder=1, ...
    useDiffraction=true, ...
    size=5000, ...
    lambdaBase=1/200, ...
    lambdaAve=1/100, ...
    lambdaSt=1/100, ...
    sourcePower=1, ...
    alpha=4, ...
    A=1, ...
    fadingMean=1, ...
    noisePower=0, ...
    doManhattan=true, ...
    pathLossNLOS=true, ...
    connectToNLOS=true, ...
    createBaseStations=true, ...
    penetrationLoss=0.1, ...
    computationNodes=100, ... Number of theoretical users we calculate for on each street
    thresholdDB=10, ... (In dB) The minimum SINR to be acceptable for the fitness test
    distBases = 15 ... Distance between each candidate base
);

hold on

simulations = 1e4;
numBins = 1000;
results = zeros(simulations, 1);

for j = 1:simulations
    data.runManhattan();
    result = SINR(data);
    results(j) = 10 * log10(result);
end


% Plot the CCDF graph for this K value
[x, y] = CCDF(results, numBins);

% Plot the graph
plot(x, y, Color="r", DisplayName="2D with NLOS and Diffraction (NLOS connect)");

data.connectToNLOS = false;

for j = 1:simulations
    data.runManhattan();
    result = SINR(data);
    results(j) = 10 * log10(result);
end

% Plot the CCDF graph for this K value
[x, y] = CCDF(results, numBins);

% Plot the graph
plot(x, y, Color="b", DisplayName="2D with NLOS and Diffraction (No NLOS connection)");
 
% Label the graph
title('Coverage probability CCDF');
xlabel('\theta');
ylabel('Probability');
legend();

hold off