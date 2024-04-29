fn main() {
    let mut even_numbers: Vec<i32> = Vec::new();
    let max_value = 300;

    for number in 1..=max_value{
        if number % 2 == 0 {
            println!("{}", max_value - number);
            even_numbers.push(number);
        }
    }

    println!("{:?}", even_numbers.iter().rev().collect::<Vec<_>>());

}
