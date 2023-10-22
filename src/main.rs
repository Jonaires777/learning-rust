use std::fmt;

struct Matrix(f32,f32,f32,f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "( {} {} )", &self.0, &self.1)?;
        write!(f, "( {} {} )", &self.2, &self.3)
    }
}

fn transpose(matrix: Matrix) -> Matrix {
    let mut transposed_matrix = matrix;

    let aux = transposed_matrix.1;
    transposed_matrix.1 = transposed_matrix.2;
    transposed_matrix.2 = aux;

    transposed_matrix
}

fn main(){
    let matrix = Matrix(1.1,1.2,2.1,2.2);
    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", transpose(matrix))
}