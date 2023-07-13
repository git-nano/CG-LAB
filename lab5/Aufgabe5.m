clear;
clc;

% Auslesen der Dateien
file = "polygon.txt";
file2 = "testpolygon.txt";
poly1 = importdata(file);
poly2 = importdata(file2);

% Ermitteln der Kreisparameter
[c, r] = KreisInPolygon(poly1);
[c2, r2] = KreisInPolygon(poly2);

% Visualisierung
subplot(2,1,1);
hold on
plot(poly1(:,1), poly1(:,2));
rectangle('Position',[c(1)-r c(2)-r r*2 r*2],'Curvature',[1 1]);
scatter(c(1), c(2), 4,"filled");
axis equal
title('polygon.txt')

subplot(2,1,2);
hold on
plot(poly2(:,1), poly2(:,2));
rectangle('Position',[c2(1)-r2 c2(2)-r2 r2*2 r2*2],'Curvature',[1 1]);
scatter(c2(1), c2(2), 4,"filled");
axis equal
title('testpolygon.txt')
