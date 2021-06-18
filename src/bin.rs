use rux::*;

fn main() {
    let x = Matrix::new(vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]]);
    let y = Matrix::new(vec![vec![11.0, 12.0], vec![13.0, 14.0], vec![15.0, 16.0]]);
    let z = Matrix::new(vec![vec![11.0, 12.0, 13.0], vec![14.0, 15.0, 16.0]]);
    println!("x: {}", x);
    println!("y: {}", y);
    println!("z: {}", z);
    // TODO: consume values rather than take as reference
    println!("z transpose = {}", !z.clone());
    println!("x * 3 = {}", x.clone() * 3);
    println!("3 * x = {}", 3 * x.clone());
    println!("x * z = {}", x.clone() * z.clone());
    println!("{}", Matrix::dot_product(&vec![3.0, 4.0, 5.0], &vec![7.0, 8.0, 9.0]));
    let x_test = Matrix::new(vec![vec![3, 4, 5]]);
    let y_test = Matrix::new(vec![vec![7, 8, 9]]);
    println!("x + y + z^t = {}", x + y + !z);
}
