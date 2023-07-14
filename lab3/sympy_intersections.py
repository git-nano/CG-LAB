import numpy as np
from sympy import Segment, Point

print("This takes a while...")

s = []
i = []

segments = np.loadtxt("../data/s_1000_10.dat")

for segment in segments:
    s.append(Segment(Point(segment[:2]),Point(segment[2:])))

for index,segment in enumerate(s):
    for j, value in enumerate(s):
        if j <= index:
            continue
        p = segment.intersection(value)
        if p:
            i.append(p)
    print(index)
    print("Intersections: ", len(i))

print(i)
print(len(i))

