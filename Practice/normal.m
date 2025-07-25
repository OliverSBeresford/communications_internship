numRealizations = 1e7;
sigma = 15; mu = 97;
numBins = 300;

% How many sigmas to view on the histogram, 96% within 5 sigma of center
k = 4;

realizations = normrnd(mu, sigma, 1, numRealizations);

figure(1)
IQ = histogram(realizations, ...
    "BinEdges", linspace(-k * sigma + mu, k * sigma + mu, numBins), ...
    "Normalization", "probability", ...
    "FaceColor", "yellow");

figure(2)
IQCDF = histogram(realizations, ...
    "BinEdges", linspace(-k * sigma + mu, k * sigma + mu, numBins), ...
    "Normalization", "cdf", ...
    "FaceColor", "yellow");

figure(3)
binCenters = IQ.BinEdges(1:end-1) + IQ.BinWidth ./ 2;
plot(binCenters, IQ.Values);
