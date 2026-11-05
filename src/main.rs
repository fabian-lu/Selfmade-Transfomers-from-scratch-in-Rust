mod matrix;
mod random_number_generator;

use matrix::Matrix;

fn main() {
    /*  //println!("test");
    let zero_matrix = Matrix::zeros(2,2);
    zero_matrix.print();
    let filled_matrix = Matrix::from_vec(2,2, vec![1.0,2.3,4.0,4.0]);
    //let failed_matrix = Matrix::from_vec(3,3,vec![2.0,2.0]);
    //print!("test after")
    filled_matrix.print();

    let last_matrix: Matrix = Matrix::from_vec(3,2, vec!
        [1.0,0.0,
         1.2,1.3,
         3.0,4.0]
    );
    last_matrix.print();

    println!("{:.1}", 2.0);

    println!("{}", last_matrix.get_value(0,0));
    println!("{}", last_matrix.get_value(0,1));
    println!("{}", last_matrix.get_value(1,0));
    println!("{}", last_matrix.get_value(1,1));
    println!("{}", last_matrix.get_value(2,0));
    println!("{}", last_matrix.get_value(2,1));*/

    /*let mut mutable_matrix: Matrix = Matrix::from_vec(2,3,
    vec![
        1.0, 2.0, 3.0,
        4.0, 5.0, 6.0
    ]);

    //mutable_matrix.print();

    mutable_matrix.set_value(1,1, 10.0);

    mutable_matrix.print();

    mutable_matrix.transpose().print()*/

    /*let mut get_col_matrix: Matrix = Matrix::from_vec(2,3,
                                                      vec![
                                                          1.0, 2.0, 3.0,
                                                          4.0, 5.0, 6.0
                                                      ]);
    get_col_matrix.print();
    println!("{:?}", get_col_matrix.get_col(2));*/

    /*let first_matrix_for_mult: Matrix = Matrix::from_vec(2, 3, vec![
        1.0,2.0,3.0,
        4.0,5.0,6.0
    ]);

    let second_matrix_for_mult: Matrix = Matrix::from_vec(3, 2, vec![
        1.0,2.0,
        3.0, 4.0,
        5.0,6.0
    ]);*/

    /*let result: Matrix = first_matrix_for_mult.matrix_mult(&second_matrix_for_mult);

    result.print()*/

    let first_matrix_for_addition: Matrix =
        Matrix::from_vec(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);

    let second_matrix_for_addition: Matrix =
        Matrix::from_vec(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);

    first_matrix_for_addition
        .add_matrix(&second_matrix_for_addition)
        .print();
    let unchunked = vec![1.0, 2.0, 3.0, 4.0, 500000.0, 6.0];
    let mut chunked = format!(
        "[{}]",
        unchunked
            .chunks(3)
            .map(|row| {
                format!(
                    "[{}]",
                    row.iter()
                        .map(|col_value| { format!("{:.2}", col_value) })
                        .collect::<Vec<String>>()
                        .join(" ")
                )
            })
            .collect::<Vec<String>>()
            .join("\n ")
    );

    println!("{}", chunked);

    // {format!("{:6.2}", row)}
}
