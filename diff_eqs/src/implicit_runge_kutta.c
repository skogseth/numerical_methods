#include "newtons_method.h"
#include "implicit_runge_kutta.h"

double (*func)(double,double);
double tn,yn,h;

double g(double x){ return x-yn-h*func(tn+h/2,yn/2+x/2); }
double dg(double x){ return 1-func(tn,x); }

void implicit_midpoint(double (*ydot)(double,double),double t0,double tf,double y0,int N,double* t,double* y){
    y[0] = y0;
    h=(tf-t0)/(N-1);
    for(int n=0; n<N; n++){ t[n] = t0 + n*h; }
    func = ydot;
    double y_est;
    for(int n=0; n<N; n++){
        tn = t[n]; yn = y[n];
        y_est = yn+h*ydot(tn,yn);
        y[n+1] = newtons_method(g,dg,y_est,0.001,10000);
    }
}
