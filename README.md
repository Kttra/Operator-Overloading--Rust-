# Operator Overloading (Rust)
 An example of a use of traits in rust. In this program we use traits to overload the +, -, and * operators. In addition, we define matrix operators.
A test case for the program would be something like this:


fn test(){<br/>
    &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;let x = Matrix::new(2, 3, &[-2, -1, 0, 1, 2, 3]);<br/>
    &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;let y = Matrix::new(2, 3, &[1, 2, 3, 4, 5, 6]);<br/>
    &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;assert_eq!(&x + &y - &y, x);<br/>
    &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;assert_eq!(format!("{}", x), "-2 -1 0\n1 2 3\n");<br/>
}
