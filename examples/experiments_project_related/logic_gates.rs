use selfmade_transformers::math::matrix::Matrix;
use selfmade_transformers::neural_network::activation::step;
use selfmade_transformers::neural_network::linear_layer::LinearLayer;

const INPUT_PAIRS: [(f32, f32); 4] = [(0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (1.0, 1.0)];

fn and_gate() {
    println!("AND GATE:");
    let weights_and_biases: LinearLayer = LinearLayer::new(
        Matrix::from_vec(1,2, vec![1.0, 1.0]),
        Matrix::from_vec(1,1, vec![-1.5])
    );

    for (x1, x2) in INPUT_PAIRS {
        let input: Matrix = Matrix::from_vec(2,1, vec![x1, x2]);
        let after_forward_pass: Matrix = weights_and_biases.forward(&input);
        let after_step: Matrix = step(&after_forward_pass);

        let output = after_step.get(0,0);
        println!("{} AND {} = {} ", x1, x2, output);
    }
    println!();
}
fn or_gate() {
    println!("OR GATE:");
    let weights_and_biases: LinearLayer = LinearLayer::new(
        Matrix::from_vec(1,2, vec![1.0, 1.0]),
        Matrix::from_vec(1,1, vec![-0.5])
    );

    for (x1, x2) in INPUT_PAIRS {
        let input: Matrix = Matrix::from_vec(2,1, vec![x1, x2]);
        let after_forward_pass: Matrix = weights_and_biases.forward(&input);
        let after_step: Matrix = step(&after_forward_pass);

        let output = after_step.get(0,0);
        println!("{} OR {} = {} ", x1, x2, output);
    }

    println!();
}

fn nand_gate() {
    println!("NAND GATE:");
    let weights_and_biases: LinearLayer = LinearLayer::new(
        Matrix::from_vec(1,2, vec![-1.0, -1.0]),
        Matrix::from_vec(1,1, vec![1.5])
    );

    for (x1, x2) in INPUT_PAIRS {
        let input: Matrix = Matrix::from_vec(2,1, vec![x1, x2]);
        let after_forward_pass: Matrix = weights_and_biases.forward(&input);
        let after_step: Matrix = step(&after_forward_pass);

        let output = after_step.get(0,0);
        println!("{} AND {} = {} ", x1, x2, output)
    }
    println!();
}


fn main() {
    and_gate();
    or_gate();
    nand_gate();

}