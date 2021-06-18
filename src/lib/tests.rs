#[cfg(test)]
mod tests {
	use crate::*;
	#[test]
	fn create_i8_matrix() {
		let data: Vec<Vec<i8>> = vec![vec![1, 2], vec![3, 4], vec![5, 6]]; 
		let data = Matrix::new(data);
		let answer: Vec<Vec<f64>> = vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]];
		let answer = Matrix::new(answer);
		assert_eq!(answer, data);
	}

	#[test]
	fn create_u8_matrix() {
		let data: Vec<Vec<u8>> = vec![vec![1, 2], vec![3, 4], vec![5, 6]]; 
		let data = Matrix::new(data);
		let answer: Vec<Vec<f64>> = vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]];
		let answer = Matrix::new(answer);
		assert_eq!(answer, data);
	}

	#[test]
	fn create_i16_matrix() {
		let data: Vec<Vec<i16>> = vec![vec![1, 2], vec![3, 4], vec![5, 6]]; 
		let data = Matrix::new(data);
		let answer: Vec<Vec<f64>> = vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]];
		let answer = Matrix::new(answer);
		assert_eq!(answer, data);
	}

	#[test]
	fn create_u16_matrix() {
		let data: Vec<Vec<u16>> = vec![vec![1, 2], vec![3, 4], vec![5, 6]]; 
		let data = Matrix::new(data);
		let answer: Vec<Vec<f64>> = vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]];
		let answer = Matrix::new(answer);
		assert_eq!(answer, data);
	}

	#[test]
	fn create_i32_matrix() {
		let data: Vec<Vec<i32>> = vec![vec![1, 2], vec![3, 4], vec![5, 6]]; 
		let data = Matrix::new(data);
		let answer: Vec<Vec<f64>> = vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]];
		let answer = Matrix::new(answer);
		assert_eq!(answer, data);
	}

	#[test]
	fn create_u32_matrix() {
		let data: Vec<Vec<u32>> = vec![vec![1, 2], vec![3, 4], vec![5, 6]]; 
		let data = Matrix::new(data);
		let answer: Vec<Vec<f64>> = vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]];
		let answer = Matrix::new(answer);
		assert_eq!(answer, data);
	}

	#[test]
	fn create_i64_matrix() {
		let data: Vec<Vec<i64>> = vec![vec![1, 2], vec![3, 4], vec![5, 6]]; 
		let data = Matrix::new(data);
		let answer: Vec<Vec<f64>> = vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]];
		let answer = Matrix::new(answer);
		assert_eq!(answer, data);
	}

	#[test]
	fn create_u64_matrix() {
		let data: Vec<Vec<u64>> = vec![vec![1, 2], vec![3, 4], vec![5, 6]]; 
		let data = Matrix::new(data);
		let answer: Vec<Vec<f64>> = vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]];
		let answer = Matrix::new(answer);
		assert_eq!(answer, data);
	}

	#[test]
	fn create_i128_matrix() {
		let data: Vec<Vec<i128>> = vec![vec![1, 2], vec![3, 4], vec![5, 6]]; 
		let data = Matrix::new(data);
		let answer: Vec<Vec<f64>> = vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]];
		let answer = Matrix::new(answer);
		assert_eq!(answer, data);
	}

	#[test]
	fn create_u128_matrix() {
		let data: Vec<Vec<u128>> = vec![vec![1, 2], vec![3, 4], vec![5, 6]]; 
		let data = Matrix::new(data);
		let answer: Vec<Vec<f64>> = vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]];
		let answer = Matrix::new(answer);
		assert_eq!(answer, data);
	}

	#[test]
	fn create_isize_matrix() {
		let data: Vec<Vec<isize>> = vec![vec![1, 2], vec![3, 4], vec![5, 6]]; 
		let data = Matrix::new(data);
		let answer: Vec<Vec<f64>> = vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]];
		let answer = Matrix::new(answer);
		assert_eq!(answer, data);
	}

	#[test]
	fn create_usize_matrix() {
		let data: Vec<Vec<usize>> = vec![vec![1, 2], vec![3, 4], vec![5, 6]]; 
		let data = Matrix::new(data);
		let answer: Vec<Vec<f64>> = vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]];
		let answer = Matrix::new(answer);
		assert_eq!(answer, data);
	}

	#[test]
	fn create_f32_matrix() {
		let data: Vec<Vec<f32>> = vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]]; 
		let data = Matrix::new(data);
		let answer: Vec<Vec<f64>> = vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]];
		let answer = Matrix::new(answer);
		assert_eq!(answer, data);
	}

	#[test]
	fn create_f64_matrix() {
		let data: Vec<Vec<f64>> = vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]]; 
		let data = Matrix::new(data);
		let answer: Vec<Vec<f64>> = vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]];
		let answer = Matrix::new(answer);
		assert_eq!(answer, data);
	}
}