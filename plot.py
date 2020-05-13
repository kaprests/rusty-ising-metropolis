import numpy as np
from matplotlib import pyplot as plt


energies = np.fromfile("./data/energies.txt", sep="\n")

print(energies[0])
print(energies[-1])

plt.title("32 x 32")
plt.xlabel("# sweeps")
plt.ylabel("Energy")
plt.plot(energies)
plt.savefig("energyplot_T2.png")
plt.show()


