#![feature(test)]

extern crate test;
use ndarray::*;
use rand::distributions::Uniform;
use option_bench::*;

#[bench]
fn raw_bench(b: &mut test::Bencher) {
    let ud = Uniform::new::<f64, f64>(0.0, 1.0);
    let mut mat = Array2::zeros((1000, 1000));
    b.iter(|| raw(&mut mat, ud))
}

#[bench]
fn option_bench(b: &mut test::Bencher) {
    let ud = Uniform::new::<f64, f64>(0.0, 1.0);
    let mut op_mat = Some(Array2::zeros((1000, 1000)));
    b.iter(|| option(&mut op_mat, ud))
}