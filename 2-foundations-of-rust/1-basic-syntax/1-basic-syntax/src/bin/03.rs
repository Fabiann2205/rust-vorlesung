fn main() {
    let input = [23, 82, 16, 45, 21, 94, 12, 34];

    let mut largest = input[0];
    let mut smallest = input[0];

    for &item in input.iter() {
        if item > largest {
            largest = item;
        }
        if item < smallest {
            smallest = item;
        }
    }

    println!("{largest} is largest and {smallest} is smallest");
}
