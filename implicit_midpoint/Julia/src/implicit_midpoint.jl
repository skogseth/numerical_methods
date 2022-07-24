include("newtons_method.jl");

function implicit_midpoint(t0::Float64, tf::Float64, y0::Float64, f::Function; h=0.01)
    N::Int = Int( (tf-t0)/h ) + 1
    t = range(t0, tf, N)
    y = zeros(N)
    y[1] = y0

    for n in 1:(N-1)
        tn, yn = t[n], y[n]
        y_est = yn + h * f(tn, yn)
        g(x) = x - yn - h * f(tn + h/2, yn/2 + x/2)
        dg(x) = 1 - f(tn, x)
        y[n+1] = newtons_method(y_est, g, dg)
    end

    return t, y
end;
