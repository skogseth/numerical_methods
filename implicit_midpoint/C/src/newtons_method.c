#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include "newtons_method.h"


double newtons_method(double (*f)(double),double (*df)(double),double x0,double tol,int max_it){
    int i = 0;
    double xn = x0;
    while(i<max_it && fabs(f(xn))>tol){
        if(df(xn)==0){ printf("Derivative hit zero\n"); break; }
        printf("Iteration: %i, xn = %f, fn = %f, dfn = %f\n",i,xn,f(xn),df(xn));
        xn -= f(xn)/df(xn);
        i++;
    }
    return xn;
}
