# Introduction

This repository provides a library for computational geometry and several use cases.

# Structure

In `cg_library` is the used library for computational geometry, with its components. Each component is tested.
Then there are the `labs`:
- `lab1`: Provides calculation of line intersections with generic points. It does not use the `cg_library`.
- `lab2`: Provides a calculation of geographical areas taken from a `svg` file. In this case, Germany and its states were subject.
- `lab3`: Provides the execution of the bently ottmann algorithm to speed up the calculation of intersection points.
- `lab4`: Provides testing with the program `qhull` to calculate the polygon hull of a set of points. This does not use the `cg_library` nor `Rust` in general.
- `lab5`: Provides a matlab implementation to calculate the inside circle of a polygon. This does not use the `cg_library` nor uses `Rust` in general.

For `lab3` python tests were provided. These can be executed using a virtual environment `venv` in the `python_venv` folder. 
Just follow the instructions after executing `bash setup-venv.sh`.

# RustDoc

Each `lab` as well as the `cg_library` is documented using RustDoc.
It can be created by using:
`cargo doc`

And opened with:
`firefox target/doc/cg_library/index.html`


# Collaboration

There was a little collaboration with team `Bissig/Grasso`. 
The results were compared and some approaches were discussed.
