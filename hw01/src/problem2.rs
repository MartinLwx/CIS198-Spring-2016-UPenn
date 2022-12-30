/// Represents a matrix in row-major order
pub type Matrix = Vec<Vec<f32>>;

/// Computes the product of the inputs `mat1` and `mat2`.
pub fn mat_mult(mat1: &Matrix, mat2: &Matrix) -> Matrix {
    let m1 = mat1.len(); 
    let n1 = mat1[0].len();
    let m2 = mat2.len();
    let n2 = mat2[0].len();

    assert_eq!(n1, m2);

    // (m1, n1) * (m2, n2) -> (m1, n2) where n1 = m2
    let mut ans = Matrix::new();

    for i in 0..m1 {
        let mut current_row = vec![0.0; n2];
        for j in 0..n2 {
            for k in 0..m2 {
                current_row[k] += mat1[i][j] * mat2[j][k];
            }
        }
        ans.push(current_row);
    }
    ans
}