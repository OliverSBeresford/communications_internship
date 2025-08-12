function [bestActivationIndex, bestDeactivationIndex] = bestCandidates(data, candidates, candidatesPerRoad, candidateSelect, baseFitness)
    % Keep track of the best and worst switches when we power a BS on/off
    bestActivation = -Inf;
    bestActivationIndex = -1;
    bestDeactivation = -Inf;
    bestDeactivationIndex = -1;

    for ii = 1:size(candidates, 1)
        isOnAnAvenue = ii <= length(data.avenues) * candidatesPerRoad;

        if candidateSelect(ii)
            % Updating numAveBases
            if isOnAnAvenue
                data.numAveBases = data.numAveBases - 1;
            end

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

            % Reverting changes to numAveBases
            if isOnAnAvenue
                data.numAveBases = data.numAveBases + 1;
            end
        else
            % Updating numAveBases
            if ii <= length(data.avenues) * candidatesPerRoad
                data.numAveBases = data.numAveBases + 1;
            end

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

            % Reverting changes to numAveBases
            if isOnAnAvenue
                data.numAveBases = data.numAveBases - 1;
            end
        end
    end
end