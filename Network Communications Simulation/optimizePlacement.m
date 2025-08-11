% Initialize the object that stores the data in intermediate steps
% Provide all the parameters to SimulationData
data = SimulationData( ...
    useNLOS=true, ...
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
    pathLossNLOS=false, ...
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

% Creates the fitness baseline for this iteration
baseFitness = fitnessValue(data, data.computationNodes, data.thresholdDB);

% Keep track of the best and worst switches when we power a BS on/off
bestActivation = -Inf;
bestActivationIndex = -1;
bestDeactivation = -Inf;
bestDeactivationIndex = -1;

for ii = 1:numCandidates
    if candidateSelect(ii)
        % Switching this one off if it's on
        candidateSelect(ii) = false;
        data.baseStations = candidateBases(candidateSelect, :);
        
        % Calculate the new fitness value after the change
        newFitness = fitnessValue(data, data.computationNodes, data.thresholdDB);
        
        % If this is the new best deactivation, update variables
        difference = newFitness - baseFitness;
        if difference > bestDeactivation
            bestDeactivationIndex = ii;
            bestDeactivation = difference;
        end
    else
        % Switching this one on if it's off
        candidateSelect(ii) = true;
        data.baseStations = candidateBases(candidateSelect, :);
        
        % Calculate the new fitness value after the change
        newFitness = fitnessValue(data, data.computationNodes, data.thresholdDB);
        
        % If this is the new best activation, update variables
        difference = newFitness - baseFitness;
        if difference > bestActivation
            bestActivationIndex = ii;
            bestActivation = difference;
        end
    end
end