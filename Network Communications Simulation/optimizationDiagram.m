% Initialize the object that stores the data in intermediate steps
% Provide all the parameters to SimulationData
data = SimulationData( ...
    useNLOS=true, ...
    diffractionOrder=1, ...
    useDiffraction=true, ...
    size=500, ...
    lambdaBase=1.5/1000, ...
    lambdaAve=7/1000, ...
    lambdaSt=7/1000, ...
    sourcePower=1, ...
    alpha=4, ...
    A=1, ...
    fadingMean=1, ...
    noisePower=0, ...
    doManhattan=true, ...
    pathLossNLOS=true, ...
    connectToNLOS=true, ...
    createBaseStations=false, ...
    penetrationLoss=0.1, ...
    computationNodes=100, ... Number of theoretical users we calculate for on each street
    thresholdDB=10, ... (In dB) The minimum SINR to be acceptable for the fitness test
    distBases = 20 ... Distance between each candidate base
);

% Number of deployable base stations
numDeployed = data.size * data.lambdaBase * (length(data.avenues) + length(data.streets));
numDeployed = round(numDeployed);

% Number of candidate base stations
candidatesPerRoad = data.size / data.distBases;
candidatesPerRoad = round(candidatesPerRoad);
numCandidates = (length(data.avenues) + length(data.streets)) * candidatesPerRoad;

% Matrix to contain the x-y coordinate pairs of the candidate base stations
candidateBases = zeros(numCandidates, 2);

% Keeps track of where we are editing in candidateBases
index = 1;

% Creates candidate base stations for avenues
for ave = data.avenues
    candidateBases(index:index + candidatesPerRoad - 1, 1) = ave;
    candidateBases(index:index + candidatesPerRoad - 1, 2) = linspace(-data.size/2, data.size/2, candidatesPerRoad);
    index = index + candidatesPerRoad;
end

% Creates candidate base stations for streets
for st = data.streets
    candidateBases(index:index + candidatesPerRoad - 1, 1) = linspace(-data.size/2, data.size/2, candidatesPerRoad);
    candidateBases(index:index + candidatesPerRoad - 1, 2) = st;
    index = index + candidatesPerRoad;
end

%% Draw all the candidates
data.baseStations = candidateBases;
figure(1);
data.drawManhattan(300, "black");

candidateSelect = false(numCandidates, 1);

% Generates random indices of candidateSelect to turn on
indices = randperm(numCandidates, numDeployed);
candidateSelect(indices) = 1;

% Update deployed base stations, with an extra one for when we switch one
data.baseStations = candidateBases(candidateSelect, :);
data.numAveBases = sum(indices <= length(data.avenues) * candidatesPerRoad);

%% Draw selected base stations
figure(2)
data.drawManhattan(300, "red");
figure(3)
data.drawManhattan(300, "red");

% Creates the fitness baseline for this iteration
baseFitness = fitnessValue(data);

[bestActivation, bestDeactivation] = bestCandidates(data, candidateBases, candidatesPerRoad, candidateSelect, baseFitness);

%% Draw the activation / deactivation
hold on
x = [candidateBases(bestDeactivation, 1), candidateBases(bestActivation, 1)];
y = [candidateBases(bestDeactivation, 2), candidateBases(bestActivation, 2)];
scatter(x, y, 300, [0 0 0; 0 0 255], Marker="x");
hold off

% Activate and deactivate 2 base stations that gave optimal performance
candidateSelect(bestActivation) = true;
candidateSelect(bestDeactivation) = false;

% Draw the new grid
data.baseStations = candidateBases(candidateSelect, :);
figure(4)
data.drawManhattan();
