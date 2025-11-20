import matplotlib.pyplot as plt
import pandas as pd

NUM_TEST = 100

df = pd.read_csv("./resultados/resultados.csv", sep=";")

fig = plt.figure()
ax = fig.add_subplot(projection="3d")

ax.set_title(
    "Média de completude $c$ de nonogramas de tamanho $NxN$ com $p$ de preenchimento"
)

ax.plot_trisurf(df["size"], df["p"], df["c"])
ax.set_zlabel("$N$")
ax.set_ylim(0, 1)
ax.set_ylabel("$c$")
ax.set_zlim(0, 1)
ax.set_zlabel("$p$")
plt.show()
