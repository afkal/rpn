//use std::collections::VecDeque;

//pub fn run(expression: &str) -> &'static str {
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

    let command_iterator = expression.split(" ");
    //let mut stack: VecDeque<&str> = commands.collect();
    //let mut numbers : VecDeque<i32> = VecDeque::new();
    //let mut commands: Vec<&str> = command_iterator.collect();
    let mut stack : Vec<i32> = Vec::new();

    //stack.push("testing");
    //println!("{:?}", stack);

    // Evaluate stack
    //let value1: i32 = stack.pop()
    
    //println!("Initial commands {:?}", commands);
    //
    for item in command_iterator {
    //while let Some(item) = commands.pop() {
        println!("--- Starting new round:");
        // Prints 3, 2, 1
        // evaluate item
        if item.trim().parse::<f64>().is_ok() {
            let num: i32 = item.parse().unwrap();
            stack.push(num);
        } else {
            if item == "+" {
                let n1 = stack.pop().expect("Value missing - error in expression!");
                let n2 = stack.pop().expect("Value missing - error in expression!");
                let sum = n1 + n2;
                println!("Plus operation ({}+{}={})", n1, n2, n1+n2);
                stack.push(sum);
            }
        }
        //println!("{item}");
        println!("Stack {:?}", stack);
        //println!("Current commands {:?}", commands);
    }
    let result = stack[0];
    stack.pop();
    return result.to_string().to_owned();
    //println!("{:?}", numbers);
    /*
    if let Some(item) = stack.pop_front() {
        println!("{}", item);
    }
    */


    //let value1: i32 = stack.pop_front()
    //"5".to_string()
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

}