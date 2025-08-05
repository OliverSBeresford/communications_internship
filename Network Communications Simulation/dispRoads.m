% Read OSM data into a geospatial table
points = readgeotable("map.osm", "Layer","points");

% Projected CRS
projCRS = projcrs(32619);

% 3. Extract latitude and longitude from each geolineshape and project
x_coords = zeros(1, 2503);
y_coords = zeros(1, 2503);

for i = 1:height(points)
    % Extract latitude and longitude from the current geolineshape
    if isprop(points.Shape(i), 'Latitude') && isprop(points.Shape(i), 'Longitude')
        lat = points.Shape(i).Latitude;
        lon = points.Shape(i).Longitude;
    else
        error('Latitude and Longitude properties not found. Adjust coordinate extraction.');
    end

    % Project the coordinates
    [x, y] = projfwd(projCRS, lat, lon);

    % Store the projected coordinates
    x_coords(i) = x;
    y_coords(i) = y;
end

% Plot the projected lines
hold on;

scatter(x_coords, y_coords, Marker=".", Color=[0 0 0]);

hold off;

xlabel('X (meters)');
ylabel('Y (meters)');
title('Projected OSM Lines');
grid on;
axis equal; % Ensure proper aspect ratio, crucial for maps

% --- INPUT ---
x = x_coords(:);  % column vector
y = y_coords(:);

% --- STEP 1: Create binary image ---
% Set image resolution (number of pixels)
img_size = 1000;

% Normalize to image grid
x_min = min(x); x_max = max(x);
y_min = min(y); y_max = max(y);

% Scale to image coordinates
x_img = round(1 + (x - x_min) / (x_max - x_min) * (img_size - 1));
y_img = round(1 + (y - y_min) / (y_max - y_min) * (img_size - 1));

% Create binary image
bw = false(img_size, img_size);
for i = 1:length(x_img)
    if x_img(i) > 0 && x_img(i) <= img_size && y_img(i) > 0 && y_img(i) <= img_size
        bw(y_img(i), x_img(i)) = true;  % Note: rows are y, cols are x
    end
end

% --- STEP 2: Hough Transform ---
[H, theta, rho] = hough(bw);

% --- STEP 3: Find peaks in Hough space ---
num_peaks = 500;  % you can increase this if needed
peaks = houghpeaks(H, num_peaks, 'Threshold', 0.1 * max(H(:)));

% --- STEP 4: Extract line segments ---
lines = houghlines(bw, theta, rho, peaks, 'FillGap', 60, 'MinLength', 150);

% --- STEP 5: Plot results ---
figure; imshow(bw); hold on;
title('Detected Lines via Hough Transform');

% Overlay original points (optional)
plot(x_img, y_img, 'g.', 'MarkerSize', 5);

% Draw detected lines
for k = 1:length(lines)
    xy = [lines(k).point1; lines(k).point2];
    plot(xy(:,1), xy(:,2), 'r-', 'LineWidth', 2);

    % Optional: mark line endpoints
    plot(xy(1,1), xy(1,2), 'yo', 'MarkerSize', 6);
    plot(xy(2,1), xy(2,2), 'co', 'MarkerSize', 6);
end

