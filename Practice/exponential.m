
numRealizations = 1e7;
lambda = 3;
numBins = 300;

realizations = exprnd(1 / lambda, 1, numRealizations);
edges = linspace(0, lambda, numBins);

figure(1)
intervals = histogram(realizations, ...
    "BinEdges", edges, ...
    "Normalization", "pdf", ...
    "FaceColor", "green");

xlabel 'Time since last event'
ylabel Probability

figure(2)
binCenters = edges(1:end-1) + intervals.BinWidth ./ 2;
plot(binCenters, intervals.Values);

xlabel 'Time since last event'
ylabel Probability

