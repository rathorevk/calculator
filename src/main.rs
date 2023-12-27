use std::env::{args, Args};

fn main() {
    let mut args: Args = args();


    let first = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let second = args.nth(0).unwrap();

    // println!("{:?} {} {} {}", args, first, operator, second);

    let first_number = first.parse::<f32>().unwrap();
    let second_number = second.parse::<f32>().unwrap();

    let result = calculcate(operator, first_number, second_number);

    println!("{:?}", output(first_number, operator, second_number, result));
}

fn calculcate(operator: char, first_number: f32, seocnd_number: f32) -> f32 {
    match operator {
        '+' => first_number + seocnd_number,
        '-' => first_number - seocnd_number,
        '/' => first_number / seocnd_number,
        '*' | 'x' | 'X' => first_number * seocnd_number,
        _ => panic!("Invalid operator!")
    }
}

fn output(first_number: f32, operator: char, second_number: f32, result: f32) -> String {
    format!("{} {} {} = {}", first_number, operator, second_number, result)
}
