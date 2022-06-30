use std::env;
//use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let val1 = &args[1];
    let val2 = &args[2];
    let val3 = &args[3];

    /*
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    */
    println!("RPN interpreter v0.0.1");
    println!("--- DEBUG: Arguments received: {} {} {}", val1, val2, val3);

    let num1: i32 = val1.trim().parse()
        .expect("please give me correct string number!");

    let num2: i32 = val2.trim().parse()
        .expect("please give me correct string number!");

    if val3 == "+" {
       let result = num1 + num2;
       println!(">>> {}", result);
       return;
    }

    if val3 == "-" {
        let result = num1 - num2;
        println!(">>> {}", result);
        return;
     }

    println!(">>> Syntax error");
}