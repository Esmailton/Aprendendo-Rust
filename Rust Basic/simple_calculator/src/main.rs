use std::io;
fn main() {
    let signals_aritimetic = ['+', '-', '/', '*'];

    println!("Enter your operation: ");

    let mut operation = String::new();

    io::stdin()
        .read_line(&mut operation)
        .expect("Failed to get operation");

    for signal in signals_aritimetic.into_iter() {
        if let Some(_result) = operation.find(signal) {

            match signal {
                '-' => {
                    let values: Vec<&str> = operation.split(signal).collect();
                    let f = values[0].trim().parse::<f32>().unwrap()
                        - values[1].trim().parse::<f32>().unwrap();
                    println!("The result is ->: {}", f);
                }
                '+' => {
                    let values: Vec<&str> = operation.split(signal).collect();
                    let f = values[0].trim().parse::<f32>().unwrap()
                        + values[1].trim().parse::<f32>().unwrap();
                    println!("The result is ->: {}", f);
                }
                '*' => {
                    let values: Vec<&str> = operation.split(signal).collect();
                    let f = values[0].trim().parse::<f32>().unwrap()
                        * values[1].trim().parse::<f32>().unwrap();
                    println!("The result is ->: {}", f);
                }
                '/' => {
                    let values: Vec<&str> = operation.split(signal).collect();
                    let f = values[0].trim().parse::<f32>().unwrap()
                        / values[1].trim().parse::<f32>().unwrap();
                    println!("The result is ->: {}", f);
                }
                _ => {
                    println!("can't understand the operation");
                }
            };
        }
    }
}