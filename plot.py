import numpy as np
import subprocess
from matplotlib import pyplot as plt


### Simulation parameters ###
num_sweeps = 100000
array_dim = 32


try:
    print("Running program")
    subprocess.run(f"target/release/ising_monte_carlo {num_sweeps} {array_dim}", shell=True)
except:
    print("Executable not found, compiling before running...")
    subprocess.run(["cargo", "run", "--release"])


### Energy plot ###
energies = np.fromfile("./data_tmp/energies.txt", sep="\n")
plt.title(f"{array_dim} x {array_dim}")
plt.xlabel("# sweeps")
plt.ylabel("Energy")
plt.plot(energies)
plt.savefig(f"energyplot_{array_dim}x{array_dim}_{num_sweeps}sweeps.png")
plt.show()


