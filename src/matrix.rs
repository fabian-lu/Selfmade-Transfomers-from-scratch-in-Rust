use crate::random_number_generator::RandomNumberGenerator;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<f32>,
}

impl Matrix {
    pub fn shape(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn zeros(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            data: vec![0.0; rows * cols],
        }
    }

    pub fn randoms(rng: &mut RandomNumberGenerator, rows: usize, cols: usize, spread: f32) -> Self {
        Self {
            rows,
            cols,
            data: rng.get_random_normal_floats(rows * cols, spread),
        }
    }

    pub fn from_vec(rows: usize, cols: usize, data: Vec<f32>) -> Self {
        assert_eq!(
            data.len(),
            rows * cols,
            "a {}x{} matrix needs {} values, but got {}",
            rows,
            cols,
            rows * cols,
            data.len()
        );
        Self { rows, cols, data }
    }

    pub fn print(&self) {
        // chunks() panics when the chunk size is 0, so an empty matrix gets its own case
        if self.data.is_empty() {
            println!("[]");
            return;
        }

        // the widest number sets the width for all numbers, so the columns line up
        let width = self
            .data
            .iter()
            .map(|value| format!("{:.2}", value).len())
            .max()
            .unwrap_or(0);

        let rows: Vec<String> = self
            .data
            .chunks(self.cols)
            .map(|row| {
                let values: Vec<String> = row
                    .iter()
                    .map(|value| format!("{:>width$.2}", value))
                    .collect();
                format!("[{}]", values.join(" "))
            })
            .collect();

        println!("[{}]", rows.join("\n "));
    }

    pub fn get(&self, row: usize, col: usize) -> f32 {
        self.data[self.index(row, col)]
    }

    fn index(&self, row: usize, col: usize) -> usize {
        assert!(
            row < self.rows,
            "row {} is out of bounds for a matrix with {} rows",
            row,
            self.rows
        );
        assert!(
            col < self.cols,
            "column {} is out of bounds for a matrix with {} columns",
            col,
            self.cols
        );

        let start_position = row * self.cols;
        start_position + col
    }

    fn row(&self, row_index: usize) -> &[f32] {
        assert!(
            row_index < self.rows,
            "row {} is out of bounds for a matrix with {} rows",
            row_index,
            self.rows
        );

        let start = row_index * self.cols;
        let end = start + self.cols;
        &self.data[start..end]
    }

    pub fn col(&self, col: usize) -> Vec<f32> {
        assert!(
            col < self.cols,
            "column {} is out of bounds for a matrix with {} columns",
            col,
            self.cols
        );

        self.data
            .iter()
            .skip(col)
            .step_by(self.cols)
            .copied()
            .collect()
    }

    pub fn set(&mut self, row: usize, col: usize, value: f32) {
        let index_to_set = self.index(row, col);
        self.data[index_to_set] = value;
    }

    pub fn transpose(&self) -> Self {
        let mut transposed_data: Vec<f32> = Vec::with_capacity(self.data.len());
        for col_index in 0..self.cols {
            transposed_data.extend(self.col(col_index));
        }
        Self::from_vec(self.cols, self.rows, transposed_data)
    }

    pub fn matrix_mult(&self, second_matrix: &Self) -> Self {
        assert_eq!(
            self.cols, second_matrix.rows,
            "cannot multiply a {}x{} matrix by a {}x{} matrix: \
             the first matrix needs as many columns as the second has rows",
            self.rows, self.cols, second_matrix.rows, second_matrix.cols
        );

        let mut new_data: Vec<f32> = Vec::with_capacity(self.rows * second_matrix.cols);
        let secon_matrix_transposed: Matrix = second_matrix.transpose();

        for row_index in 0..self.rows {
            let row = self.row(row_index);
            for col_index in 0..second_matrix.cols {
                let col = secon_matrix_transposed.row(col_index);
                let sum = dot_product(row, col);
                new_data.push(sum);
            }
        }

        Self::from_vec(self.rows, second_matrix.cols, new_data)
    }

    fn zip_with_helper(
        &self,
        second_matrix: &Self,
        operation_name: &str,
        operation: impl Fn(f32, f32) -> f32,
    ) -> Self {
        assert_eq!(
            self.shape(),
            second_matrix.shape(),
            "cannot {operation_name} a {:?} matrix and a {:?} matrix: the shapes must be the same",
            self.shape(),
            second_matrix.shape()
        );

        let new_data: Vec<f32> = self
            .data
            .iter()
            .zip(second_matrix.data.iter())
            .map(|(first_element, second_element)| operation(*first_element, *second_element))
            .collect();

        Self {
            rows: self.rows,
            cols: self.cols,
            data: new_data,
        }
    }

    pub fn add_matrix(&self, second_matrix: &Self) -> Self {
        self.zip_with_helper(second_matrix, "add", |first_element, second_element| {
            first_element + second_element
        })
    }

    pub fn element_wise_mult(&self, second_matrix: &Self) -> Self {
        self.zip_with_helper(
            second_matrix,
            "multiply",
            |first_element, second_element| first_element * second_element,
        )
    }

    fn map_with_helper(&self, operation: impl Fn(f32) -> f32) -> Self {
        let new_data: Vec<f32> = self
            .data
            .iter()
            .map(|element| operation(*element))
            .collect();

        Self {
            rows: self.rows,
            cols: self.cols,
            data: new_data,
        }
    }

    pub fn scale(&self, scale_factor: f32) -> Self {
        self.map_with_helper(|element| scale_factor * element)
    }
}

fn dot_product(first_vector: &[f32], second_vector: &[f32]) -> f32 {
    debug_assert_eq!(
        first_vector.len(),
        second_vector.len(),
        "a dot product needs two vectors of the same length, but got lengths {} and {}",
        first_vector.len(),
        second_vector.len()
    );

    first_vector
        .iter()
        .zip(second_vector.iter())
        .map(|(first_element, second_element)| first_element * second_element)
        .sum()
}
