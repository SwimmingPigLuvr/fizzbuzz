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

fn main() {
    fizzbuzz()
}
