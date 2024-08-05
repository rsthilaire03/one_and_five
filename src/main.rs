use std::io;

fn main() {
    println!("Guess a number between 1 and 5!");
    let test_variable = 2.5;
    let mut x = true;

    while x {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read line");
        let flt_input: f64 = input.trim().parse().expect("failed to parse");

        if flt_input > test_variable {
            println!("A little lower.");
        } else if flt_input < test_variable {
            println!("A little higher.");
        }
        else {
            println!("You got it!");
            x = false;
        }
    }
}
