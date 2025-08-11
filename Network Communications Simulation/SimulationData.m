classdef SimulationData < handle
    properties 
        sourcePower(1, 1) {mustBeNumeric};
        receiver {mustBeVector} = [0 0];
        alpha(1, 1) {mustBeNumeric};
        A(1, 1) {mustBeNumeric};
        fadingMean(1, 1) {mustBeNumeric};
        noisePower(1, 1) {mustBeNumeric};
        baseStations(:, 2) {mustBeMatrix, mustBeNumeric} 
        stationCount(1, 1) {mustBeNumeric}
        penetrationLoss(1, 1) {mustBeNumeric, mustBeInRange(penetrationLoss, 0, 1)};
        avenues {mustBeMatrix}
        streets {mustBeMatrix}
        lambdaBase(1, 1) {mustBeNumeric}
        lambdaAve(1, 1) {mustBeNumeric}
        lambdaSt(1, 1) {mustBeNumeric}
        useNLOS(1, 1) {mustBeNumericOrLogical}
        useDiffraction(1, 1) {mustBeNumericOrLogical}
        size(1, 1) {mustBeNumeric}
        pathLossNLOS(1, 1) {mustBeNumericOrLogical}
        diffractionOrder(1, 1) {mustBeNumeric}
        aveCounts {mustBeVector(aveCounts, 'allow-all-empties')}
        stCounts {mustBeVector(stCounts, 'allow-all-empties')}
        q90 = sqrt(0.031 / (4 * pi));
        connectToNLOS {mustBeNumericOrLogical}
    end
    methods
        function obj = SimulationData(options)
            arguments
                % Defining parameters and default values
                options.doManhattan (1, 1) {mustBeNumericOrLogical} = true;
                options.sourcePower(1, 1) {mustBeNumeric} = 1;
                options.receiver {mustBeVector} = [0 0];
                options.alpha(1, 1) {mustBeNumeric} = 4;
                options.A(1, 1) {mustBeNumeric} = 1;
                options.fadingMean(1, 1) {mustBeNumeric} = 1;
                options.noisePower(1, 1) {mustBeNumeric} = 0;
                options.baseStations(:, 2) {mustBeMatrix, mustBeNumeric} = [];
                options.stationCount(1, 1) {mustBeNumeric} = 0;
                options.penetrationLoss(1, 1) {mustBeNumeric, mustBeInRange(options.penetrationLoss, 0, 1)} = 1;
                options.avenues {mustBeMatrix} = [];
                options.streets {mustBeMatrix} = [];
                options.size(1, 1) {mustBeNumeric} = 50;
                options.lambdaBase(1, 1) {mustBeNumeric} = 0.1;
                options.lambdaAve(1, 1) {mustBeNumeric} = 1;
                options.lambdaSt(1, 1) {mustBeNumeric} = 1;
                options.useNLOS(1, 1) {mustBeNumericOrLogical} = false;
                options.useDiffraction(1, 1) {mustBeNumericOrLogical} = false;
                options.pathLossNLOS(1, 1) {mustBeNumericOrLogical} = false;
                options.diffractionOrder(1, 1) {mustBeNumeric} = 0;
                options.aveCounts {mustBeVector(options.aveCounts, 'allow-all-empties')} = [];
                options.stCounts {mustBeVector(options.stCounts, 'allow-all-empties')} = [];
                options.connectToNLOS {mustBeNumericOrLogical} = false;
            end
            % Setting the object's properties
            obj.lambdaBase = options.lambdaBase;
            obj.size = options.size;
            obj.lambdaAve = options.lambdaAve;
            obj.lambdaSt = options.lambdaSt;
            obj.useNLOS = options.useNLOS;
            obj.useDiffraction = options.useDiffraction;
            obj.sourcePower = options.sourcePower;
            obj.receiver = options.receiver;
            obj.alpha = options.alpha;
            obj.A = options.A;
            obj.fadingMean = options.fadingMean;
            obj.noisePower = options.noisePower;
            obj.penetrationLoss = options.penetrationLoss;
            obj.pathLossNLOS = options.pathLossNLOS;
            obj.diffractionOrder = options.diffractionOrder;
            obj.connectToNLOS = options.connectToNLOS;

            % If they provide true or if any values are empty, do manhattan
            doManhattan = options.doManhattan || isempty(options.avenues) || isempty(options.streets) || isempty(options.baseStations);
            doManhattan = doManhattan || isempty(options.aveCounts) || isempty(options.stCounts);
            
            % If they tried not to run the MPLP and had to anyway
            if doManhattan && ~options.doManhattan
                disp('Not enough parameters provided. Running MPLP.');
            end

            % Run MPLP or set properties to provided values
            if doManhattan
                obj.runManhattan();
            else
                obj.avenues = options.avenues;
                obj.streets = options.streets;
                obj.baseStations = options.baseStations;
                obj.stationCount = options.stationCount;
                obj.aveCounts = options.aveCounts;
                obj.stCounts = options.stCounts;
            end
        end

        % This method runs the MPLP and stores the result in properties
        function runManhattan(obj)
            manhattan(obj);
        end

        function drawManhattan(obj)
            % If you can't draw the graph, return
            if isempty(obj.baseStations) || isempty(obj.avenues) || isempty(obj.streets)
                return;
            end

            %% Draw the manhattan graph
            hold on
    
            % Each avenue (North-South) and street (East-West)
            xline(obj.avenues);
            yline(obj.streets);
            
            % Center point (receiver)
            plot(0, 0, "r o");
            
            % Limiting the viewport to a square of size squareSize
            xlim([-(obj.size / 2), obj.size / 2]);
            ylim([-(obj.size / 2), obj.size / 2]);
    
            % Draw points
            scatter(obj.baseStations(:, 1), obj.baseStations(:, 2), "blue", "x")
            
            hold off
        end
    end
end