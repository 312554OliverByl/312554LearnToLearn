/*
Learn to learn assignment.
Oliver Byl
February 26, 2019
*/

//Recommended compiler (if you don't have one installed):
//https://repl.it/languages/rust
//Just copy the contents of this file into their editor
//and run.

use std::io;

fn main() {
    println!("Welcome to Oliver Byl's Learn to Learn Assignment for ICS3U!");

    //Keep asking for problems and executing them until
    //the user has had enough.
    loop {
        match get_problem() {
            1 => problem1(),
            2 => problem2(),
            3 => problem3(),
            4 => problem4(),
            5 => problem5(),
            _ => panic!(),
        }

        if !another_problem() {
            break;
        }
    }

    println!("Thanks for stopping by!");
}

//Gets the next line of input from the user as a String.
fn get_line_of_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line.");
    input
}

//Gets the id of a problem from the user.
//Keeps running until it succeeds.
fn get_problem() -> u8 {
    println!("Please enter one of the following problems to view: J1, J2, J3, J4, or J5");
    loop {
        match get_line_of_input().trim() {
            "J1" => return 1,
            "J2" => return 2,
            "J3" => return 3,
            "J4" => return 4,
            "J5" => return 5,
            _ => println!("Please enter a valid problem name. (J1, J2, J3, J4, or J5)"),
        }
    }
}

//Decides whether or not to break out of the main loop
//or not by asking the user for an input of 'y' or 'n'.
fn another_problem() -> bool {
    println!("Would you like to see another problem? (y/n)");
    loop {
        match get_line_of_input().trim() {
            "y" => return true,
            "n" => return false,
            _ => println!("Please enter a valid response. (y/n)"),
        }
    }
}

fn problem1() {
    //Get proper input for problem.
    println!("Enter input: ");
    let mut input: [u8; 4] = [0, 0, 0, 0];
    for i in 0..4 {
        input[i] = match get_line_of_input().trim().parse::<u8>() {
            Ok(line) => line,
            Err(e) => panic!("Error with input parsing: {}", e),
        }
    }

    //Apply problem rules.
    let ignore = (input[0] == 8 || input[0] == 9) &&
                 (input[3] == 8 || input[3] == 9) &&
                 (input[1] == input[2]);

    //Output accordingly.
    if ignore {
        println!("ignore");
    } else {
        println!("answer");
    }
}

fn problem2() {
    //Get proper input for problem.
    println!("Enter input: ");
    let n: usize = match get_line_of_input().trim().parse::<usize>() {
        Ok(line) => line,
        Err(e) => panic!("Error with input parsing: {}", e),
    };

    let line1: Vec<char> = get_line_of_input().chars().collect();
    let line2: Vec<char> = get_line_of_input().chars().collect();
    
    //Check every spot and if it was occupied ("C") on
    //both days increment num_spots.
    let mut num_spots: u8 = 0;
    for i in 0..n {
        if line1[i] == 'C' && line2[i] == 'C' {
            num_spots += 1;
        }
    }

    //Output number of spots.
    println!("{}", num_spots);
}

fn problem3() {
    //Get proper input for problem.
    println!("Enter input: ");
    let input: Vec<u8> = get_line_of_input() //Get line of input.
                          .split_whitespace() //Split into characters.
                          .map(|s| s.parse().unwrap()) //Iterate and parse.
                          .collect(); //Cast into vector.

    //Compute and output.
    for line in 0..5 {
        
    }
}

fn problem4() {

}

fn problem5() {

}
