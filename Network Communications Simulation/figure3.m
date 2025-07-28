simulations = 1e5;

% This is where all the SINR results are stored
results = zeros(1, simulations);

% These are all the parameters
size = 50;
lambdaBase = 0.1;
lambdaAve = 1;
lambdaSt = 1;
plotGraph = false;
sourcePower = 1;
alpha = 4;
A = 1;
fadingMean = 1;
noisePower = 0;

% Calculate SINR (simulations) times
for ii = 1:simulations
    data = SimulationData(size=size, lambdaBase=lambdaBase, lambdaAve=lambdaAve, lambdaSt=lambdaSt, plotGraph=plotGraph, sourcePower=sourcePower, alpha=alpha, A=A, fadingMean=fadingMean, noisePower=noisePower, doManhattan=true);
    result = SINR(data);
    results(ii) = 10 * log10(result);
end

% Plot the CDF histogram
figure(1)
cdfGraph = histogram(results, 100, "Normalization", "cdf");
title('Coverage probability CDF');
xlabel('\theta');
ylabel('Probability');

% Plot the CCDF graph
figure(2)
ccdfX = cdfGraph.BinEdges(1:end - 1) + cdfGraph.BinWidth / 2;
ccdfY = 1 - cdfGraph.Values;
plot(ccdfX, ccdfY);
title('Coverage probability CCDF');
xlabel('\theta');
ylabel('Probability');
