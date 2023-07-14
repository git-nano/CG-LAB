# Process

Execute `bently_ottman`:
`cargo run`

-> This creates the file `intersection_file.dat`

# Check with python

This uses the library `line-segment-intersections` as comparison.

Install Environment:
`bash setup_venv.sh`

Source Environment:
`source .venv/bin/activate`

Test and visualize intersections (python):
`python3 visualize.py {segments_file} {intersections_file}`

Test intersections with sympy (This takes very long):
`python3 sympy_intersections.py`

-> The output for `s_1000_10.dat` is $796$

