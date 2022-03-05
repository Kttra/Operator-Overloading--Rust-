# Operator Overloading (Rust)
 An example of a use of traits in rust. In this program we use traits to overload the +, -, and * operators. In addition, we define matrix operators.
A test case for the program would be something like this:
#[test]
fn test(){
    let x = Matrix::new(2, 3, &[-2, -1, 0, 1, 2, 3]);
    let y = Matrix::new(2, 3, &[1, 2, 3, 4, 5, 6]);
    assert_eq!(&x + &y - &y, x);
    assert_eq!(format!("{}", x), "-2 -1 0\n1 2 3\n");
}