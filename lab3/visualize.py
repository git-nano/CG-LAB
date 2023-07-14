import numpy as np
import matplotlib.pyplot as plt
from matplotlib import collections as mc, use
from linesegmentintersections import bentley_ottman
import sys

segments_path = "../data/s_1000_10.dat"
intersections_path = "../data/s_1000_10_intersections.dat"
use_library = True

if len(sys.argv) == 2:
    segments_path = sys.argv[1]
elif len(sys.argv) == 3:
    use_library = False
    segments_path = sys.argv[1]
    intersections_path = sys.argv[2]
elif len(sys.argv) > 3:
    raise Exception("To many input arguments!")

segments = np.loadtxt(segments_path)
segments = segments.reshape(segments.shape[0],2,2).tolist()

lc = mc.LineCollection(segments, linewidths=0.5)

if use_library:
    intersections = np.array([[i.x,i.y] for i in bentley_ottman(segments)])
else:
    intersections = np.loadtxt(intersections_path)

fig, ax = plt.subplots()
plt.plot(intersections[:,0], intersections[:,1], 'ro', markersize=2)
ax.add_collection(lc)
plt.title("{} Line Segments with {} Intersections".format(len(segments),len(intersections)))
plt.xlabel("X-Achsis")
plt.ylabel("Y-Achsis")
plt.margins(0.1)
plt.show()
