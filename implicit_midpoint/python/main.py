import numpy as np

def newtons_method(f,df,x0,tol=10e-5,max_it=1000):
    i=0
    xn = x0
    while i<max_it and abs(f(xn))>tol:
        if df(xn)==0: print("Derivative is zero"); break
        else: xn -= f(xn)/df(xn)
        i+=1;
    return xn

def implicit_midpoint(t0,tf,y0,f,h=0.01):
    N = int((tf-t0)/h)+1
    t = np.linspace(t0,tf,N)
    y = np.zeros(N)
    y[0] = y0

    for n in range(N-1):
        tn = t[n]; yn = y[n]
        y_est = yn+h*f(tn,yn)
        g = lambda x: x-yn-h*f(tn+h/2,yn/2+x/2)
        dg = lambda x: 1-f(tn,x)
        y[n+1] = newtons_method(g,dg,y_est)

    return t,y


def main():
    t0 = 0
    tf = 10
    y0 = 5
    def f(t,x): return -0.8*x

    t,y = implicit_midpoint(t0,tf,y0,f)

    results = np.asarray([t,y]).T
    np.savetxt("data.csv", results, delimiter=",")

if __name__ == "__main__": main()
