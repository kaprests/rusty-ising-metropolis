import numpy as np
from matplotlib import pyplot as plt


energies = np.fromfile("./data/energies.txt", sep="\n")

print(energies[0])
print(energies[-1])

plt.plot(energies)
plt.show()


