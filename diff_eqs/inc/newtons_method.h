#ifndef newtons_method_h
#define newtons_method_h

double newtons_method(double (*f)(double),double (*df)(double),double x0,double tol,int max_it);

#endif
