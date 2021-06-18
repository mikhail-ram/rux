mod tests;
use num::ToPrimitive;

#[derive(Debug, Clone)]
pub struct Matrix {
    data: Vec<f64>,
    rows: usize,
    columns: usize
}

impl Matrix {
    pub fn new<T: ToPrimitive>(raw_data: Vec<Vec<T>>) -> Matrix {
        let rows = raw_data.len();
        let columns = raw_data[0].len();
        let mut data: Vec<f64> = Vec::new();
        for test_row in raw_data.iter() {
            if test_row.len() != columns {
                panic!("Please input a uniform matrix!");
            }
            data.extend(test_row.iter().map(|value| value.to_f64().unwrap()));
        }
        Matrix {
            data,
            rows,
            columns
        }
    }
    pub fn get(&self, row: usize, column: usize) -> f64 {
        if row >= self.rows || column >= self.columns {
            panic!("Please provide a valid index!");
        }
        let value = self.data[row * self.columns + column]; 
        value
    }
    pub fn dot_product(a: &Vec<f64>, b: &Vec<f64>) -> f64 {
        a.iter().zip(b.iter()).map(|(a_value, b_value)| a_value * b_value).sum()
    }
    pub fn shape(&self) -> (usize, usize) {
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

impl<T: ToPrimitive> std::ops::Add<T> for Matrix {
    type Output = Matrix;
    fn add(self, b: T) -> Matrix {
        let mut data = Vec::new();
        for self_value in self.data.iter() {
            data.push(self_value + b.to_f64().unwrap());
        }
        Matrix {
            data,
            rows: self.rows,
            columns: self.columns
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

impl<T: ToPrimitive> std::ops::Sub<T> for Matrix {
    type Output = Matrix;
    fn sub(self, b: T) -> Matrix {
        let mut data = Vec::new();
        for self_value in self.data.iter() {
            data.push(self_value - b.to_f64().unwrap());
        }
        Matrix {
            data,
            rows: self.rows,
            columns: self.columns
        }
    }
}

impl std::ops::Mul for Matrix {
	type Output = Matrix;
	fn mul(self, b: Matrix) -> Matrix {
        if self.shape().1 != b.shape().0 {
            panic!("Columns of matrix A must be equal to rows of matrix B for them to be multiplied.");
        }
        let mut data  = Vec::new();
        let b_transpose = !b;
        for self_row in self.data.chunks(self.shape().1) {
            for b_row in b_transpose.data.chunks(b_transpose.shape().1) {
                //TODO: Decide on whether to use vectors or slices
				let sum = Matrix::new(vec![self_row.to_vec()]) ^ Matrix::new(vec![b_row.to_vec()]);
                data.push(sum);
            } 
        }
        Matrix {
            data,
            rows: self.shape().0,
            columns: b_transpose.shape().0
        }
	}
}

impl<T: ToPrimitive> std::ops::Mul<T> for Matrix {
	type Output = Matrix;
	fn mul(self, b: T) -> Matrix {
		let b = b.to_f64().unwrap();
		let data = self.data.into_iter().map(|value| b * value).collect();
		Matrix {
			data,
			rows: self.rows,
			columns: self.columns
		}
	}
}

// Implementing dot-product feature using ^ operator
impl std::ops::BitXor for Matrix {
	type Output = f64;
	fn bitxor(self, b: Matrix) -> f64 {
		if (self.shape().0 != 1 || b.shape().0 != 1) || self.shape().1 != b.shape().1 {
			panic!("Matrices must only have 1 row and the same number of columns for dot product.");
		}
        self.data.into_iter().zip(b.data.into_iter()).map(|(self_value, b_value)| self_value * b_value).sum()
	}
}

// Implementing transpose feature using ! operator
impl std::ops::Not for Matrix {
	type Output = Matrix;
	fn not(self) -> Matrix {
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
}

macro_rules! impl_add_sub_multiply_for_numbers {
    ($($t:ty)*) => ($(
        impl std::ops::Sub<Matrix> for $t {
			type Output = Matrix;
			fn sub(self, b: Matrix) -> Matrix {
				let mut data = Vec::new();
				for b_value in b.data.iter() {
				data.push(self.to_f64().unwrap() - b_value);
				}
				Matrix {
				data,
				rows: b.rows,
				columns: b.columns
				}
			}
        }
		impl std::ops::Add<Matrix> for $t {
			type Output = Matrix;
			fn add(self, b: Matrix) -> Matrix {
				let mut data = Vec::new();
				for b_value in b.data.iter() {
				data.push(self.to_f64().unwrap() + b_value);
				}
				Matrix {
				data,
				rows: b.rows,
				columns: b.columns
				}
			}
		}
		impl std::ops::Mul<Matrix> for $t {
			type Output = Matrix;
			fn mul(self, b: Matrix) -> Matrix {
				let a = self.to_f64().unwrap();
				let data = b.data.into_iter().map(|b_value| a * b_value).collect();
				Matrix {
					data,
					rows: b.rows,
					columns: b.columns
				}
			}
		}
    )*)
}
impl_add_sub_multiply_for_numbers!(usize u8 u16 u32 u64 isize i8 i16 i32 i64 f32 f64);
// TODO: make implementation for all numeric types
impl std::fmt::Display for Matrix {
fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
	if f.alternate() {
		write!(f, "[{:?}", &self.data[..self.columns])?;
		for row in self.data.chunks(self.columns).skip(1) {
			write!(f, "\n {:?}", row)?;
		}
		write!(f, "]")?;
		Ok(())
	} else {
		let mut pretty_printed = Vec::new();
		for row in self.data.chunks(self.columns) {
			pretty_printed.push(row.to_vec());
		}
		write!(f, "{:?}", pretty_printed)
	}
}
}

impl PartialEq for Matrix {
	fn eq(&self, other: &Self) -> bool {
		self.data == other.data && self.shape() == other.shape()
	}
}