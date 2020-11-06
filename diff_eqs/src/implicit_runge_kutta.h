#ifndef implicit_runge_kutta_h
#define implicit_runge_kutta_h

void implicit_midpoint(double (*ydot)(double,double),double t0,double tf,double y0,int N,double* t,double* y);

#endif
