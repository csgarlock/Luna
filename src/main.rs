use luna::linalg::matrix::matrix::Matrix;

fn main() {
    let vec: Vec<f32> = vec![
        10.0, 2.0, 3.0,
        4.0, 5.0, 6.0,
        7.0, 4.0, 9.0,
    ];
    println!("{}", vec.len());
    let mut a = Matrix::new(
        3,
        3,
        vec
    );
    a.reduced_row_echelon_form();
    println!("{}", a);
}