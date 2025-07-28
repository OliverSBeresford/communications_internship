classdef SimulationData
    properties 
        sourcePower(1, 1) {mustBeNumeric};
        receiver {mustBeVector} = [0 0];
        alpha(1, 1) {mustBeNumeric};
        A(1, 1) {mustBeNumeric};
        fadingMean(1, 1) {mustBeNumeric};
        noiseP(1, 1) {mustBeNumeric};
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
                options.noiseP(1, 1) {mustBeNumeric} = 0;
                options.baseStations(:, 2) {mustBeMatrix, mustBeNumeric} = [];
                options.stationCount(1, 1) {mustBeNumeric} = 0;
                options.penetrationLoss(1, 1) {mustBeNumeric, mustBeInRange(options.penetrationLoss, 0, 1)} = 1;
                options.avenues {mustBeMatrix} = [];
                options.streets {mustBeMatrix} = [];
                options.size(1, 1) {mustBeNumeric} = 50;
                options.lambdaBase(1, 1) {mustBeNumeric} = 0.1;
                options.lambdaAve(1, 1) {mustBeNumeric} = 1;
                options.lambdaSt(1, 1) {mustBeNumeric} = 1;
                options.plotGraph(1, 1) {mustBeNumericOrLogical} = false;
                options.useNLOS(1, 1) {mustBeNumericOrLogical} = false;
                options.useDiffraction(1, 1) {mustBeNumericOrLogical} = false;
            end
            % Setting the object's properties
            obj.lambdaBase = options.lambdaBase;
            obj.lambdaAve = options.lambdaAve;
            obj.lambdaSt = options.lambdaSt;
            obj.useNLOS = options.useNLOS;
            obj.useDiffraction = options.useDiffraction;
            obj.sourcePower = options.sourcePower;
            obj.receiver = options.receiver;
            obj.alpha = options.alpha;
            obj.A = options.A;
            obj.fadingMean = options.fadingMean;
            obj.noiseP = options.noiseP;
            obj.penetrationLoss = options.penetrationLoss;
            % If they provide true or if any values are empty, do manhattan
            doManhattan = options.doManhattan || isempty(options.avenues) || isempty(options.streets) || isempty(options.baseStations);
            if doManhattan
                [obj.avenues, obj.streets, obj.baseStations, obj.stationCount] = ...
                    manhattan(options.size, ...
                        options.lambdaBase, ...
                        options.lambdaSt, ...
                        options.lambdaAve, ...
                        options.plotGraph);
            else
                obj.avenues = options.avenues;
                obj.streets = options.streets;
                obj.baseStations = options.baseStations;
                obj.stationCount = options.stationCount;
            end
        end
    end
end