// My own files
use selfmade_transformers::math::matrix::Matrix;
use selfmade_transformers::math::random_number_generator::RandomNumberGenerator;

//standard library
use std::time::Instant;

fn time_matrix_mult() {
    // in dev profile: took 1.335478213s ; in release profile: took 118.402022ms ==> about 11 times faster
    let start: Instant = Instant::now();

    let first_random_matrix: Matrix = Matrix::randoms(&mut RandomNumberGenerator::new(1),512,512, 1.0);
    let second_random_matrix: Matrix = Matrix::randoms(&mut RandomNumberGenerator::new(100),512,512, 1.0);

    let result = first_random_matrix.matrix_mult(&second_random_matrix);

    std::hint::black_box(result);

    println!("took {:?}", start.elapsed());
}

// Textbook matrix multiplication: for every result value, walk along a row of the first
// matrix and down a column of the second one. Only used to compare speeds.
fn matrix_mult_textbook(first_matrix: &Matrix, second_matrix: &Matrix) -> Matrix {
    let (rows, shared_size) = first_matrix.shape();
    let (_, cols) = second_matrix.shape();
    let mut result = Matrix::zeros(rows, cols);

    for row in 0..rows {
        for col in 0..cols {
            let mut sum = 0.0;
            for shared_index in 0..shared_size {
                sum += first_matrix.get(row, shared_index) * second_matrix.get(shared_index, col);
            }
            result.set(row, col, sum);
        }
    }

    result
}

// The same calculation with the two inner loops swapped: the innermost loop now adds to
// a different result value in every step, so the additions don't wait on each other and
// the processor can do several at once. In release mode this is much faster.
fn matrix_mult_swapped_loops(first_matrix: &Matrix, second_matrix: &Matrix) -> Matrix {
    let (rows, shared_size) = first_matrix.shape();
    let (_, cols) = second_matrix.shape();
    let mut result = Matrix::zeros(rows, cols);

    for row in 0..rows {
        for shared_index in 0..shared_size {
            let first_value = first_matrix.get(row, shared_index);
            for col in 0..cols {
                let current = result.get(row, col);
                result.set(row, col, current + first_value * second_matrix.get(shared_index, col));
            }
        }
    }

    result
}

// Runs the given work once, prints how long it took, and hands back its result
fn time_it(label: &str, work: impl Fn() -> Matrix) -> Matrix {
    let start = Instant::now();
    let result = work();
    println!("{label}: {:?}", start.elapsed());
    result
}

fn largest_difference(first_matrix: &Matrix, second_matrix: &Matrix) -> f32 {
    let (rows, cols) = first_matrix.shape();
    let mut largest = 0.0f32;
    for row in 0..rows {
        for col in 0..cols {
            let difference = (first_matrix.get(row, col) - second_matrix.get(row, col)).abs();
            largest = largest.max(difference);
        }
    }
    largest
}

fn compare_matrix_mult_versions() {
    let mut generator = RandomNumberGenerator::new(1);
    let first_matrix = Matrix::randoms(&mut generator, 512, 512, 1.0);
    let second_matrix = Matrix::randoms(&mut generator, 512, 512, 1.0);

    let mine = time_it("matrix_mult (transpose + rows)", || {
        first_matrix.matrix_mult(&second_matrix)
    });
    let textbook = time_it("textbook loops (row, col, shared)", || {
        matrix_mult_textbook(&first_matrix, &second_matrix)
    });
    let swapped = time_it("swapped loops (row, shared, col)", || {
        matrix_mult_swapped_loops(&first_matrix, &second_matrix)
    });

    // a speed comparison only means something if all versions compute the same thing
    println!(
        "largest difference to matrix_mult: textbook {}, swapped {}",
        largest_difference(&mine, &textbook),
        largest_difference(&mine, &swapped)
    );
}

fn main() {
    time_matrix_mult();
    compare_matrix_mult_versions();
}