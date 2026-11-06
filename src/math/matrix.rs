use crate::math::random_number_generator::RandomNumberGenerator;

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<f32>,
}

impl Matrix {
    #[inline]
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

    pub fn from_rows<const COLS: usize>(rows: &[[f32; COLS]]) -> Self {
        let rows_as_vec: Vec<f32> = rows.iter().flatten().copied().collect();

        Self::from_vec(rows.len(), COLS, rows_as_vec)
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

    #[inline]
    pub fn get(&self, row: usize, col: usize) -> f32 {
        self.data[self.index(row, col)]
    }

    #[inline]
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

    #[inline]
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

        let mut new_data: Vec<f32> = vec![0.0; self.rows * second_matrix.cols];

        for row_index in 0..self.rows {
            let row = self.row(row_index);

            // the part of new_data that holds this row of the result, writable
            let start = row_index * second_matrix.cols;
            let end = start + second_matrix.cols;
            let result_row = &mut new_data[start..end];

            for shared_index in 0..self.cols {
                let factor = row[shared_index];
                let second_row = second_matrix.row(shared_index);

                // result_row += factor * second_row, value by value; the steps don't
                // depend on each other, so the processor can do several at once
                for (result_value, second_value) in result_row.iter_mut().zip(second_row) {
                    *result_value += factor * second_value;
                }
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

    pub fn map_with_helper(&self, operation: impl Fn(f32) -> f32) -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;

    // compares two matrices number by number, allowing tiny rounding differences
    fn assert_matrices_close(actual: &Matrix, expected: &Matrix, tolerance: f32) {
        assert_eq!(actual.shape(), expected.shape(), "the shapes differ");
        for (position, (actual_value, expected_value)) in
            actual.data.iter().zip(expected.data.iter()).enumerate()
        {
            assert!(
                (actual_value - expected_value).abs() < tolerance,
                "the values at position {} differ: {} vs {}",
                position,
                actual_value,
                expected_value
            );
        }
    }

    // [[1, 2, 3],
    //  [4, 5, 6]]
    fn two_by_three_matrix() -> Matrix {
        Matrix::from_vec(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0])
    }

    #[test] //from vec keep shape and number in the right places
    fn from_vec_keeps_shape() {
        let test_vec = vec![1.0, 1.2, 1.3, 1.4];

        let test_matrix = Matrix::from_vec(2, 2, test_vec);

        assert_eq!(test_matrix.cols, 2);
        assert_eq!(test_matrix.rows, 2);
    }

    #[test]
    fn from_vec_stores_rows_one_after_another() {
        let matrix = two_by_three_matrix();

        assert_eq!(matrix.shape(), (2, 3));
        assert_eq!(matrix.get(0, 2), 3.0);
        assert_eq!(matrix.get(1, 0), 4.0);
    }

    #[test]
    #[should_panic(expected = "needs 6 values")]
    fn from_vec_with_wrong_length_panics() {
        Matrix::from_vec(2, 3, vec![1.0, 2.0]);
    }

    #[test]
    fn zeros_has_the_right_shape_and_only_zeros() {
        let matrix = Matrix::zeros(2, 3);

        assert_eq!(matrix.shape(), (2, 3));
        assert!(matrix.data.iter().all(|value| *value == 0.0));
    }

    #[test]
    fn set_changes_only_the_chosen_value() {
        let mut matrix = two_by_three_matrix();

        matrix.set(1, 2, 9.0);

        let expected = Matrix::from_vec(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 9.0]);
        assert_eq!(matrix, expected);
    }

    #[test]
    #[should_panic(expected = "row 2 is out of bounds")]
    fn get_with_row_out_of_bounds_panics() {
        two_by_three_matrix().get(2, 0);
    }

    #[test]
    #[should_panic(expected = "column 3 is out of bounds")]
    fn get_with_column_out_of_bounds_panics() {
        two_by_three_matrix().get(0, 3);
    }

    #[test]
    fn row_and_col_return_the_right_values() {
        let matrix = two_by_three_matrix();

        assert_eq!(matrix.row(1), &[4.0, 5.0, 6.0]);
        assert_eq!(matrix.col(1), vec![2.0, 5.0]);
    }

    #[test]
    fn transpose_matches_hand_example() {
        let transposed = two_by_three_matrix().transpose();

        let expected = Matrix::from_vec(3, 2, vec![1.0, 4.0, 2.0, 5.0, 3.0, 6.0]);
        assert_eq!(transposed, expected);
    }

    #[test]
    fn transposing_twice_gives_back_the_original() {
        let mut generator = RandomNumberGenerator::new(1);
        let matrix = Matrix::randoms(&mut generator, 3, 5, 1.0);

        assert_eq!(matrix.transpose().transpose(), matrix);
    }

    #[test]
    fn matrix_mult_matches_hand_example() {
        let first_matrix = two_by_three_matrix();
        let second_matrix = Matrix::from_vec(3, 2, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);

        let product = first_matrix.matrix_mult(&second_matrix);

        let expected = Matrix::from_vec(2, 2, vec![22.0, 28.0, 49.0, 64.0]);
        assert_eq!(product, expected);
    }

    #[test]
    fn matrix_mult_with_identity_changes_nothing() {
        let matrix = two_by_three_matrix();
        let identity = Matrix::from_vec(3, 3, vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]);

        assert_eq!(matrix.matrix_mult(&identity), matrix);
    }

    #[test]
    fn transpose_of_product_equals_product_of_transposes_in_reverse_order() {
        let mut generator = RandomNumberGenerator::new(7);
        let first_matrix = Matrix::randoms(&mut generator, 3, 4, 1.0);
        let second_matrix = Matrix::randoms(&mut generator, 4, 2, 1.0);

        let left_side = first_matrix.matrix_mult(&second_matrix).transpose();
        let right_side = second_matrix
            .transpose()
            .matrix_mult(&first_matrix.transpose());

        assert_matrices_close(&left_side, &right_side, 1e-5);
    }

    #[test]
    #[should_panic(expected = "cannot multiply a 2x3 matrix by a 2x3 matrix")]
    fn matrix_mult_with_wrong_shapes_panics() {
        two_by_three_matrix().matrix_mult(&two_by_three_matrix());
    }

    #[test]
    fn add_matrix_matches_hand_example() {
        let first_matrix = Matrix::from_vec(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        let second_matrix = Matrix::from_vec(2, 2, vec![10.0, 20.0, 30.0, 40.0]);

        let sum = first_matrix.add_matrix(&second_matrix);

        let expected = Matrix::from_vec(2, 2, vec![11.0, 22.0, 33.0, 44.0]);
        assert_eq!(sum, expected);
    }

    #[test]
    #[should_panic(expected = "cannot add")]
    fn add_matrix_with_different_shapes_panics() {
        let matrix = two_by_three_matrix();

        matrix.add_matrix(&matrix.transpose());
    }

    #[test]
    fn element_wise_mult_matches_hand_example() {
        let first_matrix = Matrix::from_vec(2, 2, vec![1.0, 2.0, 3.0, 4.0]);
        let second_matrix = Matrix::from_vec(2, 2, vec![10.0, 20.0, 30.0, 40.0]);

        let product = first_matrix.element_wise_mult(&second_matrix);

        let expected = Matrix::from_vec(2, 2, vec![10.0, 40.0, 90.0, 160.0]);
        assert_eq!(product, expected);
    }

    #[test]
    fn scale_matches_hand_example() {
        let scaled = two_by_three_matrix().scale(2.0);

        let expected = Matrix::from_vec(2, 3, vec![2.0, 4.0, 6.0, 8.0, 10.0, 12.0]);
        assert_eq!(scaled, expected);
    }

    #[test]
    fn dot_product_matches_hand_example() {
        // 1 * 4 + 2 * 5 + 3 * 6 = 32
        assert_eq!(dot_product(&[1.0, 2.0, 3.0], &[4.0, 5.0, 6.0]), 32.0);
    }

    #[test]
    fn randoms_has_the_right_shape_and_same_seed_gives_same_matrix() {
        let mut first_generator = RandomNumberGenerator::new(42);
        let mut second_generator = RandomNumberGenerator::new(42);

        let first_matrix = Matrix::randoms(&mut first_generator, 4, 3, 0.5);
        let second_matrix = Matrix::randoms(&mut second_generator, 4, 3, 0.5);

        assert_eq!(first_matrix.shape(), (4, 3));
        assert_eq!(first_matrix, second_matrix);
    }
}
