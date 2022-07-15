use crate::newtons_method::*;

pub fn implicit_midpoint<F>(
    (t0, tf): (f64, f64),
    y0: f64,
    f: F,
    n: usize,
) -> (Box<[f64]>, Box<[f64]>)
where
    F: Fn(f64, f64) -> f64,
{
    let h = (tf - t0) / (n - 1) as f64;
    let t_ptr: Box<[f64]> = (0..n).map(|x| t0 + h * (x as f64)).collect();
    let mut y_ptr: Box<[f64]> = (0..n).map(|_| 0.).collect();
    y_ptr[0] = y0;

    for i in 0..(n - 1) {
        let (ti, yi) = (t_ptr[i], y_ptr[i]);
        println!("i: {} => (ti, yi) = ({:.2}, {:.2})", i, ti, yi);
        let y_est = yi + h * f(ti, yi);

        let g = |x| x - yi - h * f(ti + h / 2., yi / 2. + x / 2.);
        let dg = |x| 1. - f(ti, x);
        y_ptr[i + 1] = newtons_method(g, dg, y_est);
    }

    println!(
        "i: {} => (ti, yi) = ({:.2}, {:.2})",
        n - 1,
        t_ptr[n - 1],
        y_ptr[n - 1]
    );

    (t_ptr, y_ptr)
}
