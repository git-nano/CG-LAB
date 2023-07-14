# Aufgabe

Lesen Sie die SVG-Datei 'DeutschlandMitStaedten.svg' und ermitteln Sie die Flächen der einzelnen Bundesländer (bezüglich der in der Datei verwendeten Skala). Am Ende der Datei befinden sich Koordinaten von Städten, Versuchen Sie herauszufinden (bzw. lassen Sie das Ihren Rechner machen ;-), in welchem Bundesland diese jeweils liegen.

Auch für diese Aufgabe sollten Sie versuchen, mich in der Ausarbeitung zu überzeugen (bzw. mir möglichst einfach nachvollziehbar machen), warum die Flächen, die Sie ermittelt haben, korrekt sind.

# Introduction

This lab uses the `cg_library` for polygon implementation. `Polygon2D` is used for the area/border definition of a state and a country. It reads a `svg` file and reorders the state capitals to the states. This implementation also calculates the area of each state in two formats, for one it uses the the standard polygon area calculated in the pixel environment, but also scales these areas to the real world area of Germany in this case.

# Usage

`cargo run --release`

For further usage, see implementation in the `main` function.


