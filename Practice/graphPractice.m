x = linspace(-10 * pi, 10 * pi, 100);
y = 10 * sin(1 / 5 * x) + x;

figure(1);
% Plot a sine function, its inverse, and y = x
hold on
plot(x, y, "r-diamond", "DisplayName", "f(x)")
plot(y, x, "g-diamond", "DisplayName", "f^-1(x)")
plot([-50, 50], [-50, 50], "b", "DisplayName", "y = x")
grid on
% To see inverses better
axis equal
legend("show", "Location", "southeast");
hold off

figure(2)

% Tiled layout with two graphs next to each other
tiledlayout(1, 2)

% First graph is a circle
nexttile
thetas = linspace(0, 2 * pi, 100);
x = cos(thetas);
y = sin(thetas);
plot(x, y, "b", "DisplayName", "Circle");
% So that it is a circle and not an oval
axis equal
grid on
legend("show", "Location", "southeast");

% Second graph is a weird circle
nexttile
plot(x .^ pi, y, "b", "DisplayName", "Weird Circle");
xline(0, "DisplayName", "Line");
legend("show", "Location", "southeast");
axis equal
grid on

% Gaussian graph
figure(3)
hold on
% Gaussian parameters
sigma = 1; mu = 1;
xsize = 5;
% Center around the center of the gaussian
x = linspace(-xsize + mu, xsize + mu, 100);
pdf1 = 1 / (sigma * sqrt(2 * pi)) .* exp(-0.5 .* (x - mu).^2 / (sigma^2));
cdf1 = 0.5 .* (1 + erf((x - mu) / (sigma * sqrt(2))));
% Plotting everything and putting a line where the center is
plot(x, pdf1, "b-", "DisplayName", "PDF");
plot(x, cdf1, "r-", "DisplayName", "CDF");
xline(mu, "DisplayName", "mu");
yline(0.5, "DisplayName", "y = 0.5")
legend("show", "Location", "southeast");

% 3d
figure(4)
x = linspace(-5, 5, 100);
y = linspace(-5, 5, 100)';
z = x .^ 2 + y .^ 2;
surf(x, y, z)
