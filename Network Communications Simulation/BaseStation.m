classdef BaseStation < handle
    properties 
        x(1, 1) {mustBeNumeric};
        y(1, 1) {mustBeNumeric};
        z(1, 1) {mustBeNumeric};
        power(1, 1) {mustBeNumeric} = 1;
    end

    methods
        % Constructor function
        function obj = BaseStation(x, y, z, power)
            obj.x = x;
            obj.y = y;
            obj.z = z;
            obj.power = power;
        end
    end
end