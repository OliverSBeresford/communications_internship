% Read OSM data into a geospatial table
GT = readgeotable("map.osm", "Layer","points");

% Projected CRS
projCRS = projcrs(32619);

% 3. Extract latitude and longitude from each geolineshape and project
x_coords = cell(1, 2503);
y_coords = cell(1, 2503);

for i = 1:height(GT)
    % Extract latitude and longitude from the current geolineshape
    if isprop(GT.Shape(i), 'Latitude') && isprop(GT.Shape(i), 'Longitude')
        lat = GT.Shape(i).Latitude;
        lon = GT.Shape(i).Longitude;
    else
        error('Latitude and Longitude properties not found. Adjust coordinate extraction.');
    end

    % Project the coordinates
    [x, y] = projfwd(projCRS, lat, lon);

    % Store the projected coordinates
    x_coords{i} = x;
    y_coords{i} = y;
end

% Plot the projected lines
hold on;

scatter(cell2mat(x_coords), cell2mat(y_coords), Marker=".", Color=[0 0 0]);

hold off;

xlabel('X (meters)');
ylabel('Y (meters)');
title('Projected OSM Lines');
grid on;
axis equal; % Ensure proper aspect ratio, crucial for maps