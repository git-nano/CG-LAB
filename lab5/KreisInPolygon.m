function [c, r] = KreisInPolygon(Polygon)
% Input: Punkte des Polygons
% Output: mittels linearer Programmierung berechneter Mittelpunkt c und 
% Radius r des größten einbeschriebenen Kreis

% Methodik: Es werden die Normalenvektoren auf allen Liniensegmenten des
% Polygons bestimmt, mit denen die Distanz d des Mittelpunktes zum Liniensegment
% berechnet und als Anforderung d >= r für die Optimierung übergeben wird


    % Definition der Start- und Endpunkte jedes Liniensegments
    Startpunkte = Polygon;
    % Sind erster und letzter Punkt des Polygons gleich, kann für das letzte 
    % Segment kein Normalenvektoren bestimmt werden, weshalb der Punkt aus 
    % der Liste gelöscht wird
    if Startpunkte(end,:) == Startpunkte(1,:)
        Startpunkte(end,:) = [];
    end
    
    % Endpunkt des Liniensegments von Startpunkt n ist Element n+1 der Liste
    Endpunkte = circshift(Startpunkte,1);
    
    anzahl_punkte = size(Startpunkte,1);
    
    % Erstellen der Normalenvektoren zu jedem Liniensegment
    N = (Endpunkte - Startpunkte) * [0 1; -1 0];
    % Normalisieren
    norm = sqrt(sum(N.^2,2));
    N = N./[norm, norm];
    
    
    % Der Abstand d vom Kreismittelpunkt c zu Startpunkte(i,:) ist
    % d = dot(N(i,:), c - Startpunkte(i,:)). 
    % Damit der Kreis innerhalb des Polygons liegt, muss für alle Kanten
    % d >= r gelten.
    % Für jede Kante des Polygons wird d als Slack Variable eingeführt,
    % sodass gilt
    % dot(N(i,:),C-A(i,:)) - S(i) == 0
    % x wird entsprechend als [c, r, S(i)] definiert, wodurch sich Aeq und beq 
    % mit N*C + 0*r - S = N*A ergeben
    Aeq = [N, zeros(anzahl_punkte,1), -eye(anzahl_punkte)];
    beq = sum(N.*Startpunkte,2);
    
    % Mit den Ungleichungen wir der Radius durch die Slack Variablen beschränkt
    % r <= s(i)
    A = [zeros(anzahl_punkte,2), ones(anzahl_punkte,1), -eye(anzahl_punkte)];
    b = zeros(anzahl_punkte,1);
    % Ziel ist ein größtmöglicher Radius r -> zu minimierende Funktion f=-r
    f = zeros(anzahl_punkte+3,1);
    f(3) = -1;
    % Lösen des Minimierungsproblem
    result = linprog(f,A,b,Aeq,beq);
    % Aus den Ergebnissen für x werden die Kreisparameter zurückgegeben
    c = result(1:2)';
    % sind die Punkte clockwise angeordnet und nicht wie üblich counterclockwise, 
    % so liegt das Polygoninnere rechts der Kanten. Dadurch zeigt der 
    % Normalenvektor aus dem Polygon raus und r wird als negativ berechnet. 
    % Als Betrag kann die Funktion für beide Sortierungen arbeiten ohne die
    % übergebenen Daten überprüfen zu müssen
    r = abs(result(3)); 
end