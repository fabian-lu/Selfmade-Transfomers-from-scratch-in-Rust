pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<f32>,
}

impl Matrix {
    pub fn zeros(rows: usize, cols: usize) -> Matrix {
        Matrix {
            rows,
            cols,
            data: vec![0.0; rows * cols],
        }
    }

    pub fn from_vec(rows: usize, cols: usize, vector: Vec<f32>) -> Matrix {
        assert_eq!(
            vector.len(),
            rows * cols,
            "Length of the Vector does not fit into the matrix."
        );
        Matrix {
            rows,
            cols,
            data: vector,
        }
    }

    pub fn print(&self) {
        print!("[");
        for row_index in 0..self.rows {
            print!("[");
            let start: usize = row_index * self.cols;
            for col_index in 0..self.cols {
                let index: usize = start + col_index;
                if col_index == self.cols - 1 {
                    print!("{}", self.data[index]);
                } else {
                    print!("{}, ", self.data[index]);
                }
            }
            print!("]");
            if row_index != self.rows - 1 {
                println!()
            }
        }
        print!("]");
        println!();
    }

    pub fn get_value(&self, row: usize, col: usize) -> f32 {
        assert!(row < self.rows, "Row out of bounds.");
        assert!(col < self.cols, "Column out of bounds.");

        let start_position = row * self.cols;
        let position_value = start_position + col;
        self.data[position_value]
    }

    fn get_row(&self, row: usize) -> &[f32] {
        let start = row * self.cols;
        let end = start + self.cols;
        &self.data[start..end]
    }

    pub fn get_col(&self, col: usize) -> Vec<f32> {
        let transposed_matrix: Matrix = self.transpose();
        let start = col * transposed_matrix.cols;
        let end = start + transposed_matrix.cols;
        transposed_matrix.data[start..end].to_vec()
    }

    pub fn set_value(&mut self, row: usize, col: usize, value: f32) {
        assert!(row < self.rows, "Row out of bounds.");
        assert!(col < self.cols, "Column out of bounds.");

        let start_position = row * self.cols;
        let position_value = start_position + col;
        self.data[position_value] = value
    }

    pub fn transpose(&self) -> Self {
        let mut transposed_matrix = Self::zeros(self.cols, self.rows);
        for row_index in 0..self.rows {
            for col_index in 0..self.cols {
                let old_value = self.get_value(row_index, col_index);
                transposed_matrix.set_value(col_index, row_index, old_value);
            }
        }

        transposed_matrix
    }

    fn dot_product(first_vec: &[f32], second_vec: &[f32]) -> f32 {
        first_vec
            .iter()
            .zip(second_vec.iter())
            .map(|(first_element, second_element)| first_element * second_element)
            .sum()
    }

    pub fn matrix_mult(&self, second_matrix: &Self) -> Self {
        assert!(
            self.cols == second_matrix.rows,
            "Indices not matching for multiplication"
        );

        let mut new_matrix: Matrix = Self::zeros(self.rows, second_matrix.cols);
        for row_index in 0..new_matrix.rows {
            for col_index in 0..new_matrix.cols {
                let row = self.get_row(row_index);
                let col = second_matrix.get_col(col_index);

                let sum = Self::dot_product(row, &col);
                new_matrix.set_value(row_index, col_index, sum);
            }
        }

        new_matrix
    }

    pub fn add_matrix(&self, second_matrix: &Self) -> Self {
        assert_eq!(self.cols, second_matrix.cols, "Columns do not match!");
        assert_eq!(self.rows, second_matrix.rows, "Rows do not match!");

        let new_data: Vec<f32> = self
            .data
            .iter()
            .zip(second_matrix.data.iter())
            .map(|(first_item, second_item)| first_item + second_item)
            .collect();

        Matrix {
            rows: self.rows,
            cols: self.cols,
            data: new_data,
        }
    }

    pub fn element_wise_mult(&self, second_matrix: &Self) -> Self {
        assert_eq!(self.cols, second_matrix.cols, "Columns do not match!");
        assert_eq!(self.rows, second_matrix.rows, "Rows do not match!");

        let new_data: Vec<f32> = self
            .data
            .iter()
            .zip(second_matrix.data.iter())
            .map(|(first_item, second_item)| first_item * second_item)
            .collect();

        Matrix {
            rows: self.rows,
            cols: self.cols,
            data: new_data,
        }
    }

    pub fn scale(&self, scale_factor: f32) -> Self {
        let new_data = self.data.iter().map(|item| scale_factor * item).collect();

        Matrix {
            rows: self.rows,
            cols: self.cols,
            data: new_data,
        }
    }
}

/*[
[0.0 0.1]
[1.0 1.1]
[2.0 2.1]
]
= [0.0 0.1] [1.0 1.1] [2.0 2.1]
  0    1     2    3     4   5 */
