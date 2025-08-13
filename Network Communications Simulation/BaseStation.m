classdef BaseStation < handle
    properties 
        x(1, 1) {mustBeNumeric};
        y(1, 1) {mustBeNumeric};
        z(1, 1) {mustBeNumeric};
        power(1, 1) {mustBeNumeric} = 1;
    end
end