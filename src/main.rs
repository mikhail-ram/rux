use std::f64;

pub trait num {}

impl num for u8 {}
impl num for u16 {}
impl num for u32 {}
impl num for u64 {}
impl num for u128 {}
impl num for usize {}
impl num for i8 {}
impl num for i16 {}
impl num for i32 {}
impl num for i64 {}
impl num for i128 {}
impl num for isize {}

//TODO: Generic data type
#[derive(Debug, Clone)]
struct Matrix {
    data: Vec<f64>,
    rows: usize,
    columns: usize
}

impl Matrix {
    fn new(raw_data: Vec<Vec<f64>>) -> Matrix {
        let rows = raw_data.len();
        let columns = raw_data[0].len();
        let mut data: Vec<f64> = Vec::new();
        for test_row in raw_data.iter() {
            if test_row.len() != columns {
                panic!("Please input a uniform matrix!");
            }
            data.extend(test_row);
        }
        Matrix {
            data,
            rows,
            columns
        }
    }
    fn get(&self, row: usize, column: usize) -> f64 {
        if row >= self.rows || column >= self.columns {
            panic!("Please provide a valid index!");
        }
        let value = self.data[row * self.columns + column]; 
        value
    }
    fn multiply(a: &Matrix, b: &Matrix) -> Matrix {
        if a.shape().1 != b.shape().0 {
            panic!("Columns of matrix A must be equal to rows of matrix B for them to be multiplied.");
        }
        let mut data  = Vec::new();
        let b_transpose = b.transpose();
        for a_row in a.data.chunks(a.shape().1) {
            for b_row in b_transpose.data.chunks(b_transpose.shape().1) {
                //TODO: Decide on whether to use vectors or slices
                let sum = Matrix::dot_product(&a_row.to_vec(), &b_row.to_vec());
                data.push(sum);
            } 
        }
        Matrix {
            data,
            rows: a.shape().0,
            columns: b.shape().1
        }
    }
    fn dot_product(a: &Vec<f64>, b: &Vec<f64>) -> f64 {
        a.iter().zip(b.iter()).map(|(a_value, b_value)| a_value * b_value).sum()
    }
    fn transpose(&self) -> Matrix {
        let mut data = Vec::new();
        for column in 0..self.columns {
            for row in self.data.chunks(self.columns) {
                data.push(row[column])
            }
        }
        Matrix {
            data,
            rows: self.columns,
            columns: self.rows
        }
    }
    fn shape(&self) -> (usize, usize) {
        (self.rows, self.columns)
    }
}

impl std::ops::Add for Matrix {
    type Output = Matrix;
    fn add(self, b: Matrix) -> Matrix {
        if self.shape() != b.shape() {
            panic!("Matrices to be added must be the same shape!");
        }
        let mut data = Vec::new();
        for (index, self_value) in self.data.iter().enumerate() {
            data.push(self_value + b.data[index]);
        }
        Matrix {
            data,
            rows: self.rows,
            columns: self.columns
        }
    }
}

impl std::ops::Add<f64> for Matrix {
    type Output = Matrix;
    fn add(self, b: f64) -> Matrix {
        let mut data = Vec::new();
        for self_value in self.data.iter() {
            data.push(self_value + b);
        }
        Matrix {
            data,
            rows: self.rows,
            columns: self.columns
        }
    }
}

impl std::ops::Add<Matrix> for f64 {
    type Output = Matrix;
    fn add(self, b: Matrix) -> Matrix {
        let mut data = Vec::new();
        for b_value in b.data.iter() {
            data.push(self + b_value);
        }
        Matrix {
            data,
            rows: b.rows,
            columns: b.columns
        }
    }
}

impl std::ops::Sub for Matrix {
    type Output = Matrix;
    fn sub(self, b: Matrix) -> Matrix {
        if self.shape() != b.shape() {
            panic!("Matrices to be subtracted must be the same shape!");
        }
        let mut data = Vec::new();
        for (index, self_value) in self.data.iter().enumerate() {
            data.push(self_value - b.data[index]);
        }
        Matrix {
            data,
            rows: self.rows,
            columns: self.columns
        }
    }
}

impl std::ops::Sub<f64> for Matrix {
    type Output = Matrix;
    fn sub(self, b: f64) -> Matrix {
        let mut data = Vec::new();
        for self_value in self.data.iter() {
            data.push(self_value - b);
        }
        Matrix {
            data,
            rows: self.rows,
            columns: self.columns
        }
    }
}

impl std::ops::Sub<Matrix> for f64 {
    type Output = Matrix;
    fn sub(self, b: Matrix) -> Matrix {
        let mut data = Vec::new();
        for b_value in b.data.iter() {
            data.push(self - b_value);
        }
        Matrix {
            data,
            rows: b.rows,
            columns: b.columns
        }
    }
}

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut pretty_printed = Vec::new();
        for row in self.data.chunks(self.columns) {
            pretty_printed.push(row.to_vec());
        }
        write!(f, "{:?}", pretty_printed)
    }
}

fn main() {
    let x: Vec<Vec<f64>> = vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]];
    let y: Vec<Vec<f64>> = vec![vec![11.0, 12.0], vec![13.0, 14.0], vec![15.0, 16.0]];
    let z: Vec<Vec<f64>> = vec![vec![11.0, 12.0, 13.0], vec![14.0, 15.0, 16.0]];
    let x = Matrix::new(x);
    let y = Matrix::new(y);
    let z = Matrix::new(z);
    println!("x: {}", x);
    println!("y: {}", y);
    println!("z: {}", z);
    // TODO: Implement {#} printing syntax
    println!("z transpose = {}", z.transpose());
    let multiplied = Matrix::multiply(&x, &z);
    println!("{}", multiplied);
    println!("{}", Matrix::dot_product(&vec![3.0, 4.0, 5.0], &vec![7.0, 8.0, 9.0]));
}
