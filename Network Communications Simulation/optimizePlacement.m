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
    distBases = 15 ... Distance between each candidate base
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

candidateSelect = false(numCandidates, 1);

% Generates random indices of candidateSelect to turn on
indices = randperm(numCandidates, numDeployed);
candidateSelect(indices) = 1;

% Update deployed base stations, with an extra one for when we switch one
data.baseStations = candidateBases(candidateSelect, :);
data.numAveBases = sum(indices <= length(data.avenues) * candidatesPerRoad);

% Creates the fitness baseline for this iteration
baseFitness = fitnessValue(data);

% Condition for the while loop
noticeableDifference = true;
results = ones(10, 1) * -Inf;

while noticeableDifference
    [bestActivation, bestDeactivation] = bestCandidates(data, candidateBases, candidatesPerRoad, candidateSelect, baseFitness);
    
    % Activate and deactivate 2 base stations that gave optimal performance
    candidateSelect(bestActivation) = true;
    candidateSelect(bestDeactivation) = false;

    % Updating numAveBases
    data.numAveBases = sum(candidateSelect(1:length(data.avenues) * candidatesPerRoad));
    
    % If there is no noticeable change in the fitness, stop
    newFitness = fitnessValue(data);
    
    % If there is no variance in the last 10 results, stop
    if results(10) ~= -Inf && std(results) / mean(results) < 0.02
        noticeableDifference = false;
    end
    
    % Update last 10 results
    results(2:10) = results(1:9);
    results(1) = newFitness;
    
    disp(baseFitness + " " + newFitness)

    baseFitness = newFitness;
end

data.baseStations = candidateBases(candidateSelect, :);

displayFitness(data, 500);
coverageProbability(data, 1e4);
