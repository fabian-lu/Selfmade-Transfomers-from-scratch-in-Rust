use crate::math::matrix::Matrix;

pub struct LinearLayer {
    weights: Matrix,
    bias: Matrix,
}

impl LinearLayer {
    pub fn new(weights: Matrix, bias: Matrix) -> Self {
        assert_eq!(
            weights.shape().0,
            bias.shape().0,
            "the bias needs one row per neuron: the weights have shape {:?}, so the bias needs {} rows, but it has shape {:?}",
            weights.shape(),
            weights.shape().0,
            bias.shape()
        );
        assert_eq!(
            bias.shape().1,
            1,
            "the bias must be a column vector with 1 column, but it has shape {:?}",
            bias.shape()
        );

        Self { weights, bias }
    }

    pub fn forward(&self, input_vector: &Matrix) -> Matrix {
        self.weights
            .matrix_mult(input_vector)
            .add_matrix(&self.bias)
    }
}
