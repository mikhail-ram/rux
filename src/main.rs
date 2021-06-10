struct Matrix {
    data: Vec<Vec<f64>>
}

fn add(x: [[u32; 2]; 3], y: [[u32; 2]; 3]) {
    let mut sum = [[0; 2]; 3]; 
    println!("{:?}\n+\n{:?}", x, y);
    for (row_index, row_x) in x.iter().enumerate() {
        let row_y = y[row_index];
        for (column_index, value_x) in row_x.iter().enumerate() {
            let value_y = row_y[column_index];
            sum[row_index][column_index] = value_x + value_y;
        }
    }
    println!("{:?}", sum);
}

fn add_vec(x: Vec<Vec<u32>>, y: Vec<Vec<u32>>) {
    let sum: Vec<Vec<u32>> = Vec::new();
    println!("{:?}\n+\n{:?}", x, y);
    for (row_index, row_x) in x.iter().enumerate() {
        println!("{:?}", row_x);
        /*let row_y = y[row_index];
        for (column_index, value_x) in row_x.iter().enumerate() {
            let value_y = row_y[column_index];
            sum[row_index][column_index] = value_x + value_y;
        }*/
    }
    println!("{:?}", sum);
}

fn main() {
    let x = [[1, 2],
             [3, 4],
             [5, 6]];
    let y = [[11, 12],
             [13, 14],
             [15, 16]];
    add(x, y);
    println!("Hello, world!");
}
