
// use std::{time::Duration, thread};
use owo_colors::OwoColorize;
fn fizzbuzz() {
    for f in 1..100 {
        match (f % 3, f % 5) {
            (0, 0) => {println!("{}", ("fizzbuzz").truecolor(0, 255, 255))}
            (0, _) => {println!("{}", ("fizz").truecolor(0, 255, 0))}
            (_, 0) => {println!("{}", ("buzz").truecolor(255, 0, 255))}
            _ => {
                println!("{}", f.cyan().dimmed())
            }
        }
    }
}

fn soda_bee(upto: i32) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for f in 1..=upto {
        result.push(match (f % 3, f % 5, f) {
            (0, 0, _) => "🥤🐝".into(),
            (_, _, 69) => "nice".to_string(),
            (0, _, _) => "🥤".into(),
            (_, 0, _) => "🐝".into(),
            (_, _, _) => f.to_string()
        })
    }
    result
}

fn main() {
    // fizzbuzz()

    println!("{}", ("Starting FIZZBUZZ app").yellow().bold());
    println!("{}", ("please input a number :").yellow().dimmed());
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("can't read");
    let upto = input.trim().parse().unwrap();

   

    let vector = soda_bee(upto);
    for j in vector.into_iter(){
                println!("{}", j.bright_green());
    }

}
