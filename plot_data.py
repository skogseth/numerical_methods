import numpy as np
import matplotlib.pyplot as plt

data = np.loadtxt('diff_eqs/data.txt', delimiter=',')
t = data[:,0]
y_num = data[:,1]

N = len(t); y_sol = np.zeros(N);
for n in range(N): y_sol[n] = 5*np.exp(-0.5*t[n])

plt.plot(t,y_num,label="y_num")
plt.plot(t,y_sol,label="y_sol")
plt.xlabel("t")
plt.ylabel("y")
plt.legend()
plt.show()