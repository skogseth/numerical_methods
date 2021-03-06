use crate::newtons_method::*;

pub type Array = Box<[f64]>;

pub fn implicit_midpoint<F>((t0, tf): (f64, f64), y0: f64, f: F, n: usize) -> (Array, Array)
where
    F: Fn(f64, f64) -> f64,
{
    let t_ptr = linspace(t0, tf, n);
    let mut y_ptr = zeros(n);
    y_ptr[0] = y0;

    let h = (tf - t0) / (n - 1) as f64;

    for i in 0..(n - 1) {
        let (ti, yi) = (t_ptr[i], y_ptr[i]);
        let y_est = yi + h * f(ti, yi);

        let g = |x| x - yi - h * f(ti + h / 2., yi / 2. + x / 2.);
        let dg = |x| 1. - f(ti, x);
        y_ptr[i + 1] = newtons_method(g, dg, y_est);
    }

    (t_ptr, y_ptr)
}

fn linspace(start: f64, stop: f64, steps: usize) -> Array {
    let h = (stop - start) / (steps - 1) as f64;
    (0..steps).map(|x| start + h * (x as f64)).collect()
}

fn zeros(n: usize) -> Array {
    (0..n).map(|_| 0.).collect()
}
