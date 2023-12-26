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


fn main() {
    println!("Hello, world!");
}
