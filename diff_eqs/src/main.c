#include <stdio.h>
#include <math.h>
#include "newtons_method.h"
#include "implicit_runge_kutta.h"

double f(double x){ return 2*x*x-x;}//x*x-x; }
double df(double x){ return 4*x-1; }

double ydot(double t, double y){ return -0.5*y; }
void run_implicit_midpoint();

int main(){
    run_implicit_midpoint();

    return 0;
}


void run_implicit_midpoint(){
    int N = 501;
    double t[N],y[N];
    double y0 = 5,t0 = 0, tf=5;

    implicit_midpoint(ydot,t0,tf,y0,N,t,y);

    printf("%f",y[0]);

    FILE* fp;
    fp = fopen("data.txt","w");
    if(fp == NULL){ printf("file can't be opened\n"); return; }
    for(int n=0; n<N; n++){ fprintf(fp,"%f,%f\n",t[n],y[n]); }
    fclose(fp);
}
