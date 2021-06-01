use ndarray::*;

/// Calculation of Matrix
pub fn raw(mat: &mut Array2<usize>) {
    Zip::indexed(mat)
        .for_each(|(i, j), a| {
            *a += i + j;
        });
}

/// Calculation of Option<Matrix>
pub fn option(op_mat: &mut Option<Array2<usize>>) {
    match op_mat.as_mut() {
        Some(mat) => {
            Zip::indexed(mat)
                .for_each(|(i, j), a| {
                    *a += i + j;
                });
        }
        None => {}
    }
}