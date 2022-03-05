#![allow(unused_variables)]
#![allow(unused_imports)]
use std::{ops, fmt};
//use std::ops::Add;
use std::ops::{Add, Sub, Mul};

#[test]
fn test() {
    let x = Matrix::new(2, 3, &[-2, -1, 0, 1, 2, 3]);
    let y = Matrix::new(2, 3, &[1, 2, 3, 4, 5, 6]);
    assert_eq!(&x + &y, x + y);
    //assert_eq!(format!("{}", x), "-2 -1 0\n1 2 3\n");
}
#[test]
fn test1(){
    let x = Matrix::new(2, 3, &[-2, -1, 0, 1, 2, 3]);
    let y = Matrix::new(2, 3, &[1, 2, 3, 4, 5, 6]);
    assert_eq!(&x + &y - &y, x);
}
#[test]
fn test2(){
        let x = Matrix::new(2, 3, &[0,3,5,5,5,2]);
        let y = Matrix::new(3, 2, &[3,4,3,-2,4,-2]);
        assert_eq!(&x * &y, x * y);
}
#[test]
fn test3(){
    let x = Matrix::new(2, 3, &[-2, -1, 0, 1, 2, 3]);
    let y = Matrix::new(2, 3, &[1, 2, 3, 4, 5, 6]);
    assert_eq!(&x + &y, x.add(y));
}
#[test]
fn test4(){
    let x = Matrix::new(3, 2, &[2, 3, 4, 5, 6, 7]);
    let y = Matrix::new(3, 2, &[1, 1, 1, 1, 1, 1]);
    //assert_eq!(&x + &y, x.add(y));
    println!("{}", &x.sub(&y));
}
#[test]
fn test5(){
        let x = Matrix::new(2, 3, &[0,3,5,5,5,2]);
        let y = Matrix::new(3, 2, &[3,4,3,-2,4,-2]);
        println!("{}", x * y);
        //assert_eq!(&x * &y, x * y);
}

fn main(){
        let x = Matrix::new(1, 3, &[3,4,2]);
        let y = Matrix::new(3, 4, &[13, 9, 7, 15, 8, 7, 4, 6, 6, 4, 0, 3]);
        // x + y;
        // assert_eq!(&x + &y - &y, x);
        //assert_eq!(&x + &y, x + y);
        //assert_eq!(format!("{}", x), "-2 -1 0\n1 2 3\n");
        println!("{}", x * y);
}

#[derive(PartialEq, Debug)]
pub struct Matrix<T> {
    /// Stores elements in [row-major order](https://en.wikipedia.org/wiki/Row-major_order)
    data: Vec<T>,
    /// Number of rows
    row: usize,
    /// Number of columns
    col: usize,
}

impl<T: Copy> Matrix<T> {
    /// Creates a new matrix of `row` rows and `col` columns, and initializes
    /// the matrix with the elements in `values` in row-major order.
    pub fn new(row: usize, col: usize, values: &[T]) -> Matrix<T> {
        
        Matrix{data: values.to_vec(), row: row, col: col}
    }

    /// Creates a new, empty matrix of `row` rows and `col` columns.
    /// `data` contains no element.
    pub fn new_empty(row: usize, col: usize) -> Matrix<T> {
        Matrix{data: Vec::new(), row: 0, col: 0}
        //unimplemented!();
    }

    /// Returns a shared reference to `data`
    pub fn data(&self) -> &Vec<T> {
        &self.data
    }

    /// Returns a mutable reference to `data`
    pub fn mut_data(&mut self) -> &mut Vec<T> {
        &mut self.data
    }

    /// Returns the number of rows and columns in the first and second
    /// elements of the tuple, respectively.
    pub fn size(&self) -> (usize, usize) {
        (self.row, self. col)
    }
}

impl<'a, T: ops::Add<Output = T> + Copy> ops::Add for &'a Matrix<T> {
    type Output = Matrix<T>;

    /// Returns the sum of `self` and `rhs`. If `self.row != rhs.row || self.col != rhs.col`, panic.
    fn add(self, rhs: Self) -> Self::Output {
        if self.row != rhs.row || self.col != rhs.col{
            panic!("");
        }
        else{
            let mut matrix: Vec<T> = Vec::new();
            let num_elements = rhs.row * rhs.col;
            for i in 0..num_elements{
                matrix.push(self.data[i] + rhs.data[i]);
            }
            Matrix{row: rhs.row, col: rhs.col, data: matrix}
            
        }
    }
}

impl<'a, T: ops::Add<Output = T> + Copy> ops::Add<Matrix<T>> for &'a Matrix<T> {
    type Output = Matrix<T>;

    /// Returns the sum of `self` and `rhs`. If `self.row != rhs.row || self.col != rhs.col`, panic.
    fn add(self, rhs: Matrix<T>) -> Self::Output {
        if self.row != rhs.row || self.col != rhs.col{
            panic!("");
        }
        else{
            let mut matrix: Vec<T> = Vec::new();
            let num_elements = rhs.row * rhs.col;
            for i in 0..num_elements{
                matrix.push(self.data[i] + rhs.data[i]);
            }
            Matrix{row: rhs.row, col: rhs.col, data: matrix}
            
        }
    }
}

impl<T: ops::Add<Output = T> + Copy> ops::Add for Matrix<T> {
    type Output = Self;

    /// Returns the sum of `self` and `rhs`. If `self.row != rhs.row || self.col != rhs.col`, panic.
    fn add(self, rhs: Self) -> Self::Output {
        if self.row != rhs.row || self.col != rhs.col{
            panic!("");
        }
        else{
            let mut matrix: Vec<T> = Vec::new();
            let num_elements = rhs.row * rhs.col;
            for i in 0..num_elements{
                matrix.push(self.data[i] + rhs.data[i]);
            }
            Matrix{row: rhs.row, col: rhs.col, data: matrix}
            
        }
    }
}

impl<'a, T: ops::Add<Output = T> + Copy> ops::Add<&'a Self> for Matrix<T> {
    type Output = Self;

    /// Returns the sum of `self` and `rhs`. If `self.row != rhs.row || self.col != rhs.col`, panic.
    fn add(self, rhs: &Self) -> Self::Output {
        if self.row != rhs.row || self.col != rhs.col{
            panic!("");
        }
        else{
            let mut matrix: Vec<T> = Vec::new();
            let num_elements = rhs.row * rhs.col;
            for i in 0..num_elements{
                matrix.push(self.data[i] + rhs.data[i]);
            }
            Matrix{row: rhs.row, col: rhs.col, data: matrix}
        }
    }
}

impl<'a, T: ops::Sub<Output = T> + Copy> ops::Sub for &'a Matrix<T> {
    type Output = Matrix<T>;

    /// Returns the subtraction of `rhs` from `self`. If `self.row != rhs.row || self.col != rhs.col`, panic.
    fn sub(self, rhs: Self) -> Self::Output {
        if self.row != rhs.row || self.col != rhs.col{
            panic!("");
        }
        else{
            let mut matrix: Vec<T> = Vec::new();
            let num_elements = rhs.row * rhs.col;
            for i in 0..num_elements{
                matrix.push(self.data[i] - rhs.data[i]);
            }
            Matrix{row: rhs.row, col: rhs.col, data: matrix}
        }
    }
}

impl<'a, T: ops::Sub<Output = T> + Copy> ops::Sub<Matrix<T>> for &'a Matrix<T> {
    type Output = Matrix<T>;

    /// Returns the subtraction of `rhs` from `self`. If `self.row != rhs.row || self.col != rhs.col`, panic.
    fn sub(self, rhs: Matrix<T>) -> Self::Output {
        if self.row != rhs.row || self.col != rhs.col{
            panic!("");
        }
        else{
            let mut matrix: Vec<T> = Vec::new();
            let num_elements = rhs.row * rhs.col;
            for i in 0..num_elements{
                matrix.push(self.data[i] - rhs.data[i]);
            }
            Matrix{row: rhs.row, col: rhs.col, data: matrix}
            
        }
    }
}

impl<T: ops::Sub<Output = T> + Copy> ops::Sub for Matrix<T> {
    type Output = Self;

    /// Returns the subtraction of `rhs` from `self`. If `self.row != rhs.row || self.col != rhs.col`, panic.
    fn sub(self, rhs: Self) -> Self::Output {
        if self.row != rhs.row || self.col != rhs.col{
            panic!("");
        }
        else{
            let mut matrix: Vec<T> = Vec::new();
            let num_elements = rhs.row * rhs.col;
            for i in 0..num_elements{
                matrix.push(self.data[i] - rhs.data[i]);
            }
            Matrix{row: rhs.row, col: rhs.col, data: matrix}
            
        }
    }
}

impl<'a, T: ops::Sub<Output = T> + Copy> ops::Sub<&'a Self> for Matrix<T> {
    type Output = Self;

    /// Returns the subtraction of `rhs` from `self`. If `self.row != rhs.row || self.col != rhs.col`, panic.
    fn sub(self, rhs: &Self) -> Self::Output {
        if self.row != rhs.row || self.col != rhs.col{
            panic!("");
        }
        else{
            let mut matrix: Vec<T> = Vec::new();
            let num_elements = rhs.row * rhs.col;
            for i in 0..num_elements{
                matrix.push(self.data[i] - rhs.data[i]);
            }
            Matrix{row: rhs.row, col: rhs.col, data: matrix}
            
        }
    }
}

impl<'a, T: ops::Add<Output = T> + ops::Mul<Output = T> + Copy> ops::Mul for &'a Matrix<T> {
    type Output = Matrix<T>;

    /// Returns the multiplication of `self` by `rhs`. If `self.col != rhs.row`, panic.
    fn mul(self, rhs: Self) -> Self::Output {
        if self.col != rhs.row{
            panic!("");
        }
        else{
            let mut matrix: Vec<T> = Vec::new();
            let mut temp_matrix: Vec<T> = Vec::new();
            let num_elements = rhs.row * rhs.col;
            let mut index = 0;

            for i in 0..rhs.col{
                for j in 0..self.col{
                    temp_matrix.push(self.data[j] * rhs.data[index]);
                    index = index +  2;
                }
                let length = temp_matrix.len() - 1;
                for num in 0..length{
                    let num1 = temp_matrix.pop().unwrap();
                    let num2 = temp_matrix.pop().unwrap();
                    temp_matrix.push(num1 + num2);
                    index = 1;
                }
                matrix.push(temp_matrix.pop().unwrap());
            }

            index = 0;
            for i in 0..rhs.col{
                for j in self.col..num_elements{
                    temp_matrix.push(self.data[j] * rhs.data[index]);
                    index += 2;
                }
                let length = temp_matrix.len() - 1;
                for num in 0..length{
                    let num1 = temp_matrix.pop().unwrap();
                    let num2 = temp_matrix.pop().unwrap();
                    temp_matrix.push(num1 + num2);
                    index = 1;
                }
                matrix.push(temp_matrix.pop().unwrap());
            }
            Matrix{row: rhs.row, col: rhs.col, data: matrix}
        }
    }
}

impl<'a, T: ops::Add<Output = T> + ops::Mul<Output = T> + Copy> ops::Mul<Matrix<T>> for &'a Matrix<T> {
    type Output = Matrix<T>;

    /// Returns the multiplication of `self` by `rhs`. If `self.col != rhs.row`, panic.
    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        if self.col != rhs.row{
            panic!("");
        }
        else{
            let mut matrix: Vec<T> = Vec::new();
            let mut temp_matrix: Vec<T> = Vec::new();
            let num_elements = rhs.row * rhs.col;
            let mut index = 0;

            for i in 0..rhs.col{
                for j in 0..self.col{
                    temp_matrix.push(self.data[j] * rhs.data[index]);
                    index = index +  2;
                }
                let length = temp_matrix.len() - 1;
                for num in 0..length{
                    let num1 = temp_matrix.pop().unwrap();
                    let num2 = temp_matrix.pop().unwrap();
                    temp_matrix.push(num1 + num2);
                    index = 1;
                }
                matrix.push(temp_matrix.pop().unwrap());
            }

            index = 0;
            for i in 0..rhs.col{
                for j in self.col..num_elements{
                    temp_matrix.push(self.data[j] * rhs.data[index]);
                    index += 2;
                }
                let length = temp_matrix.len() - 1;
                for num in 0..length{
                    let num1 = temp_matrix.pop().unwrap();
                    let num2 = temp_matrix.pop().unwrap();
                    temp_matrix.push(num1 + num2);
                    index = 1;
                }
                matrix.push(temp_matrix.pop().unwrap());
            }
            Matrix{row: rhs.row, col: rhs.col, data: matrix}
        }
    }
}

impl<T: ops::Add<Output = T> + ops::Mul<Output = T> + Copy> ops::Mul for Matrix<T> {
    type Output = Self;

    /// Returns the multiplication of `self` by `rhs`. If `self.col != rhs.row`, panic.
    fn mul(self, rhs: Self) -> Self::Output {
        if self.col != rhs.row{
            panic!("");
        }
        else{
            let mut matrix: Vec<T> = Vec::new();
            let mut temp_matrix: Vec<T> = Vec::new();
            let num_elements = rhs.row * rhs.col;
            let mut index = 0;
            let mut reset = 0;

            // for i in 0..rhs.col{
            //     for j in 0..self.col{
            //         temp_matrix.push(self.data[j] * rhs.data[index]);
            //         index = index +  rhs.col;
            //     }
            //     let length = temp_matrix.len() - 1;
            //     reset += 1;
            //     for num in 0..length{
            //         let num1 = temp_matrix.pop().unwrap();
            //         let num2 = temp_matrix.pop().unwrap();
            //         temp_matrix.push(num1 + num2);
            //         index = reset;
            //     }
            //     matrix.push(temp_matrix.pop().unwrap());
            // }

            // index = 0;
            // reset = 0;
            // for i in 0..rhs.col{
            //     for j in self.col..num_elements{
            //         temp_matrix.push(self.data[j] * rhs.data[index]);
            //         index += rhs.col;
            //     }
            //     let length = temp_matrix.len() - 1;
            //     reset += 1;
            //     for num in 0..length{
            //         let num1 = temp_matrix.pop().unwrap();
            //         let num2 = temp_matrix.pop().unwrap();
            //         temp_matrix.push(num1 + num2);
            //         index = reset;
            //     }
            //     matrix.push(temp_matrix.pop().unwrap());
            // }
            Matrix{row: rhs.row, col: rhs.col, data: matrix}
        }       
    }
}

impl<'a, T: ops::Add<Output = T> + ops::Mul<Output = T> + Copy> ops::Mul<&'a Self> for Matrix<T> {
    type Output = Self;

    /// Returns the multiplication of `self` by `rhs`. If `self.col != rhs.row`, panic.
    fn mul(self, rhs: &Self) -> Self::Output {
        // if self.col != rhs.row{
        //     panic!("");
        // }
        // else{
        //     let mut matrix: Vec<T> = Vec::new();
        //     let mut temp_matrix: Vec<T> = Vec::new();
        //     let num_elements = rhs.row * rhs.col;
        //     let mut index = 0;

        //     for i in 0..rhs.col{
        //         for j in 0..self.col{
        //             temp_matrix.push(self.data[j] * rhs.data[index]);
        //             index = index +  2;
        //         }
        //         let length = temp_matrix.len() - 1;
        //         for num in 0..length{
        //             let num1 = temp_matrix.pop().unwrap();
        //             let num2 = temp_matrix.pop().unwrap();
        //             temp_matrix.push(num1 + num2);
        //             index = 1;
        //         }
        //         matrix.push(temp_matrix.pop().unwrap());
        //     }

        //     index = 0;
        //     for i in 0..rhs.col{
        //         for j in self.col..num_elements{
        //             temp_matrix.push(self.data[j] * rhs.data[index]);
        //             index += 2;
        //         }
        //         let length = temp_matrix.len() - 1;
        //         for num in 0..length{
        //             let num1 = temp_matrix.pop().unwrap();
        //             let num2 = temp_matrix.pop().unwrap();
        //             temp_matrix.push(num1 + num2);
        //             index = 1;
        //         }
        //         matrix.push(temp_matrix.pop().unwrap());
        //     }
        //     Matrix{row: rhs.row, col: rhs.col, data: matrix}
        // }
        unimplemented!();
    }
}

impl<T: fmt::Display> fmt::Display for Matrix<T> {
    /// Formats the matrix as follows:
    /// * Writes each row on a separate line. No empty lines before or after any row.
    /// * On each row, writes each element followed by a single space, except no space following the last element of the row.
    /// Outputs using `write!(f, ...)`.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let num_ele = self.row * self.col;
        for i in 0..num_ele{
            if i == num_ele - 2{
                write!(f, "{} ", self.data[num_ele - 2])?;
                break;
            }
            else if (i+1)%(self.col) == 0{
                write!(f, "{}\n", self.data[i])?;
            }
            else{
                write!(f, "{} ", self.data[i])?;
            }
        }
        write!(f, "{}", self.data[num_ele - 1])
    }
}