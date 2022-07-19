using Plots

g(x) = x^2 + 4*x + 3

x = range(-10, 10, length=1001)
y = g.(x)

plot(x, y)
gui()
