function newtons_method(x0::Float64, f::Function, df::Function; reltol=1e-4, max_iter=1e6)
    xp = x0
    xn = Inf
    for i in 1:max_iter
        xn = xp - f(xp)/df(xp)
        if f(xn)/f(xp) < reltol;
            break;
        end
        xp = xn
    end
    return xn
end;
