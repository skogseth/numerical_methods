use csv::Writer;
use std::error::Error;
use std::process;

mod implicit_midpoint;
mod newtons_method;

use implicit_midpoint::*;

fn main() {
    let t = (0., 10.);
    let y0 = 5.;
    let f = |_: f64, y: f64| -0.8 * y;
    let n = 1001;
    let (t_ptr, y_ptr) = implicit_midpoint(t, y0, f, n);

    if let Err(err) = to_csv(t_ptr, y_ptr) {
        println!("{}", err);
        process::exit(1);
    }
}

fn to_csv(t_ptr: Array, y_ptr: Array) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path("data.csv")?;
    for i in 0..t_ptr.len() {
        let point = [t_ptr[i].to_string(), y_ptr[i].to_string()];
        wtr.write_record(&point)?;
    }
    wtr.flush()?;
    Ok(())
}
