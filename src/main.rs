use std::io;
use std::io::Write;

// Write a program to evenly divide pizzas. Prompt for the
// number of people, the number of pizzas, and the number of
// slices per pizza. Ensure that the number of pieces comes out
// even. Display the number of pieces of pizza each person
// should get. If there are leftovers, show the number of leftover
// pieces

// A program to evenly divide pizzas based on how many people, and slices per pizza.
// Inputs: n_people, n_pizza, n_slices.
// Process: 
// Output: slice_per_person, leftovers

// How many people? 8
// How many pizzas do you have? 2
// How many slices per pizza? 8

// 8 people sharing 2 pizzas with 8 slices each.
// Each person gets 2 slices of pizza.
// There are 0 leftover slices of pizza.

// calculate_slices_per_person(n_people: 8, n_pizza: 2, n_slices: 8) -> (slice_per_person, leftovers)

fn calculate_slices(n_people: i64, n_pizza: i64, n_slices: i64) -> (i64, i64) {
    let total_slices: i64 = n_pizza * n_slices;
    let slice_per_person: i64 = total_slices / n_people;
    let leftover_slices: i64 = total_slices % n_people;
    (slice_per_person, leftover_slices)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_slices() {
        assert_eq!(calculate_slices(8,2,8), (2,0));
        assert_eq!(calculate_slices(7,3,8), (3,3));
        assert_eq!(calculate_slices(10,4,6), (2,4));
        assert_eq!(calculate_slices(12,3,6), (1,6));
        assert_eq!(calculate_slices(9,5,8), (4,4));
    }
}

fn get_input<T: std::str::FromStr>(prompt: &str) -> T {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim().parse() {
            Ok(value) => break value,
            Err(_) => println!("Invalid input. Please try again."),
        }
    }
}

fn main() {
    print!("How many people? ");
    let mut n_people = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut n_people).expect("Failed to read input");
    let n_people: i64 = n_people.trim().parse().expect("Please enter a valid number");

    print!("How many pizzas do you have? ");
    let mut n_pizza = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut n_pizza).expect("Failed to read input");
    let n_pizza: i64 = n_pizza.trim().parse().expect("Please enter a valid number");

    print!("How many slices per pizza? ");
    let mut n_slices = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut n_slices).expect("Failed to read input");
    let n_slices: i64 = n_slices.trim().parse().expect("Please enter a valid number");

    let (slice_per_person, leftover_slices) = calculate_slices(n_people, n_pizza, n_slices);

    println!("{} people sharing {} pizzas with {} slices each.", n_people, n_pizza, n_slices);
    println!("Each person gets {} {} of pizza.", slice_per_person, if slice_per_person > 1 { "slices" } else { "slice" } );
    println!("There are {} leftover slices of pizza.", leftover_slices);

}