use crate::math::matrix::Matrix;

pub fn step(input: &Matrix) -> Matrix {
    input.map_with_helper(|element| {if element >= 0.0 {1.0} else {0.0}})
}