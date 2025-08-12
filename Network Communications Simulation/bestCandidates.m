function [bestActivationIndex, bestDeactivationIndex] = bestCandidates(data, candidates, numCandidates, candidateSelect, baseFitness)
    % Keep track of the best and worst switches when we power a BS on/off
    bestActivation = -Inf;
    bestActivationIndex = -1;
    bestDeactivation = -Inf;
    bestDeactivationIndex = -1;

    for ii = 1:numCandidates
        if candidateSelect(ii)
            % Switching this one off if it's on
            candidateSelect(ii) = false;
            data.baseStations = candidates(candidateSelect, :);
            
            % Calculate the new fitness value after the change
            newFitness = fitnessValue(data);
            
            % If this is the new best deactivation, update variables
            difference = newFitness - baseFitness;
            if difference > bestDeactivation
                bestDeactivationIndex = ii;
                bestDeactivation = difference;
            end
            
            % Reverting base station to its original state
            candidateSelect(ii) = true;
        else
            % Switching this one on if it's off
            candidateSelect(ii) = true;
            data.baseStations = candidates(candidateSelect, :);
            
            % Calculate the new fitness value after the change
            newFitness = fitnessValue(data);
            
            % If this is the new best activation, update variables
            difference = newFitness - baseFitness;
            if difference > bestActivation
                bestActivationIndex = ii;
                bestActivation = difference;
            end
            
            % Reverting base station to its original state
            candidateSelect(ii) = false;
        end
    end
end