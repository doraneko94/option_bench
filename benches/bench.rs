#![feature(test)]

extern crate test;
use ndarray::*;
use option_bench::*;

#[bench]
fn raw_bench(b: &mut test::Bencher) {
    let mut mat = Array2::zeros((1000, 1000));
    b.iter(|| raw(&mut mat))
}

#[bench]
fn option_bench(b: &mut test::Bencher) {
    let mut op_mat = Some(Array2::zeros((1000, 1000)));
    b.iter(|| option(&mut op_mat))
}