//use std::collections::VecDeque;

pub fn run(expression: &str) -> String {

    // implement stack as vector
    //let mut stack : VecDeque<&str> = VecDeque::new();

    //let mut numbers : VecDeque<i32> = VecDeque::new();

    /*
    // iterate command string
    for item in expression.split(" ") {
        stack.push_back(item);
    }
    */

    let commands = expression.split(" ");
    //let mut commands: Vec<&str> = command_iterator.collect();
    let mut stack : Vec<i32> = Vec::new();
    
    //println!("Initial commands {:?}", commands);
    //
    for item in commands {
        println!("--- Starting new round:");
        println!("Processing item: {}", item);
        if item.trim().parse::<f64>().is_ok() {
            let num: i32 = item.parse().unwrap();
            stack.push(num);
        } else {
            if item == "+" {
                let n1 = stack.pop().expect("Value missing - error in expression!");
                let n2 = stack.pop().expect("Value missing - error in expression!");
                let result = n1 + n2;
                println!("Plus operation ({}+{}={})", n1, n2, n1+n2);
                stack.push(result);
            }
            if item == "-" {
                let n1 = stack.pop().expect("Value missing - error in expression!");
                let n2 = stack.pop().expect("Value missing - error in expression!");
                let result = n2 - n1;
                println!("Plus operation ({}+{}={})", n1, n2, n2-n1);
                stack.push(result);                
            }
            if item == "*" {
                let n1 = stack.pop().expect("Value missing - error in expression!");
                let n2 = stack.pop().expect("Value missing - error in expression!");
                let result = n2 * n1;
                println!("Plus operation ({}+{}={})", n1, n2, n1*n2);
                stack.push(result);                
            }
        }
        //println!("{item}");
        println!("Stack {:?}", stack);
        //println!("Current commands {:?}", commands);
    }
    let result = stack[0];
    stack.pop();
    return result.to_string().to_owned();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adding_two_integers() {
        let command = "2 3 +";
        //evaluate(command);
        assert_eq!("5", run(command));
    }

    #[test]
    fn adding_other_integers() {
        let command = "4 3 +";
        //evaluate(command);
        assert_eq!("7", run(command));
    }

    #[test]
    fn long_addition() {
        let command = "2 3 4 5 6 + + + +";
        //evaluate(command);
        assert_eq!("20", run(command));
    }

    #[test]
    fn substract_two_integers() {
        let command = "5 4 -";
        //evaluate(command);
        assert_eq!("1", run(command));
    }

    #[test]
    fn long_addition_and_substraction() {
        let command = "2 3 4 5 6 + - + +";
        //evaluate(command);
        assert_eq!("-2", run(command));
    }

    #[test]
    fn long_addition_substraction_multiplication() {
        let command = "2 3 4 5 6 + - * +";
        //evaluate(command);
        assert_eq!("-19", run(command));
    }
}