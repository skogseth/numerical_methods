const TOL: f64 = 10e-9;
const MAX_ITER: usize = 1000;

#[allow(non_camel_case_types)]
pub fn newtons_method<F, dF>(f: F, df: dF, x0: f64) -> f64
where
    F: Fn(f64) -> f64,
    dF: Fn(f64) -> f64,
{
    let mut xi = x0;

    for _ in 1..MAX_ITER {
        let numerator = f(xi);
        if numerator < TOL {
            break;
        }

        let denominator = df(xi);
        assert!(denominator != 0.);

        xi -= numerator / denominator;
    }

    xi
}
