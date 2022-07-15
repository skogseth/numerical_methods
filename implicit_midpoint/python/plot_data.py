import numpy as np
import matplotlib.pyplot as plt

data = np.loadtxt('data.csv', delimiter=',')
t = data[:,0]
y_num = data[:,1]

N = len(t); y_sol = np.zeros(N);
for n in range(N): y_sol[n] = y_num[0]*np.exp(-0.8*t[n])

plt.plot(t,y_num,label="y_num")
plt.plot(t,y_sol, '--',label="y_sol")
plt.xlabel("t")
plt.ylabel("y")
plt.legend()
plt.show()
