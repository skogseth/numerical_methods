use crate::newtons_method::*;

pub fn implicit_midpoint<F>(t_ptr: Box<[f64]>, y0: f64, f: F) -> Box<[f64]>
where
    F: Fn(f64, f64) -> f64,
{
    let n = t_ptr.len();
    solve_implicit_midpoint(t_ptr, y0, f, n)
}

fn solve_implicit_midpoint<F>(t_ptr: Box<[f64]>, y0: f64, f: F, n: usize) -> Box<[f64]>
where
    F: Fn(f64, f64) -> f64,
{
    let h = 0.01;
    assert_eq!(n, t_ptr.len());
    let mut y_ptr: Box<[f64]> = (0..n).map(|_| 0.).collect();
    y_ptr[0] = y0;

    for i in 0..(n - 1) {
        let (ti, yi) = (t_ptr[i], y_ptr[i]);
        let y_est = yi + h * f(ti, yi);

        let g = |x| x - yi - h * f(ti + h / 2., yi / 2. + x / 2.);
        let dg = |x| 1. - f(ti, x);
        y_ptr[i + 1] = newtons_method(g, dg, y_est);
    }

    y_ptr
}
