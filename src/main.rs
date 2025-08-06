use luna::linalg::matrix::matrix::Matrix;

fn main() {
    let a = Matrix::new(
        3,
        2,
        vec![
            0.0, 1.0,
            2.0, 3.0,
            4.0, 5.0,
        ]
    );
    let b = Matrix::new(
        2,
        3,
        vec![
            0.0, 1.0, 2.0,
            3.0, 4.0, 5.0,
        ]
    );
    println!("{}", &a * &b);
}