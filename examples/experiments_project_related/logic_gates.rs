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
        println!("{} NAND {} = {} ", x1, x2, output)
    }
    println!();
}

fn xor_gate() {
    //xor = (OR) AND (NAND)
    println!("XOR GATE:");

    for (x1, x2) in INPUT_PAIRS {
        let input: Matrix = Matrix::from_vec(2,1, vec![x1, x2]);
        let hidden_layer: LinearLayer = LinearLayer::new(
            Matrix::from_vec(2,2,
                             vec![
                                 1.0, 1.0, // or
                                 -1.0, -1.0 // nand
                             ]
            ),
            Matrix::from_vec(2,1,
                             vec![
                                 -0.5, // or
                                 1.5 // nand
                             ]
            )
        );
        let after_hidden_layer: Matrix = hidden_layer.forward(&input);
        let after_hidden_step: Matrix = step(&after_hidden_layer);

        // and layer
        let output_layer: LinearLayer = LinearLayer::new(
            Matrix::from_vec(1,2, vec![1.0, 1.0]),
            Matrix::from_vec(1,1, vec![-1.5])
        );

        let output: Matrix = output_layer.forward(&after_hidden_step);
        let output_after_step: Matrix = step(&output);

        println!("{} XOR {} = {} ", x1, x2, output_after_step.get(0,0));
    }
}


fn main() {
    and_gate();
    or_gate();
    nand_gate();
    xor_gate();
}