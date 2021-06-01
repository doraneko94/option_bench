use ndarray::*;
use rand::distributions::{Uniform, Distribution};

/// Calculation of Matrix
pub fn raw(mat: &mut Array2<f64>, ud: Uniform<f64>) {
    Zip::from(mat)
        .for_each(|a| {
            *a = ud.sample(&mut rand::thread_rng());
        });
}

/// Calculation of Option<Matrix>
pub fn option(op_mat: &mut Option<Array2<f64>>, ud: Uniform<f64>) {
    match op_mat.as_mut() {
        Some(mat) => {
            Zip::from(mat)
                .for_each(|a| {
                    *a = ud.sample(&mut rand::thread_rng());
                });
        }
        None => {}
    }
}