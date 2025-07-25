
numRealizations = 1e7;
% Events per hour on average
lambda = 3;
numBins = 50;

realizations = poissrnd(lambda, 1, numRealizations);
edges = 0:(5 * lambda);

figure(1)
intervals = histogram(realizations, ...
    "BinEdges", edges, ...
    "Normalization", "pdf", ...
    "FaceColor", "green");

xlabel 'How many events happened in an hour'
ylabel Probability
