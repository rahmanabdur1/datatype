fn main() {
    let int_value: i32 =42;
    let float_value: f64 = 3.45;
    let is_rust_fun: bool = true;
    let char_value: char = 'R';

    let tuple_example = (500,30, 50, 'z');
    let array_example = [1,3,4,5,6,7];

    println!("Integer: {}", int_value);
    println!("Float: {}", float_value);
    println!("Boolean: {}", is_rust_fun);
    println!("Character: {}", char_value);
    println!("Tuple: {:?}", tuple_example);
    println!("Array: {:?}", array_example);
    println!("Hello, world!");

    another_function(10, 'g');
    StatementsExpressions_function();
    
}



// Function
fn another_function(x: i32, unit_label:char) {
    println!("another fn: {x} {unit_label}");
}


// Statements and Expressions

fn StatementsExpressions_function( ) {
      let y={ 
        let x =3;
        x+1
      };
      println!("the value {y}")
}



//Functions with Return Values


// fn five(x: i32)-> i32{
//     x+50
// }

