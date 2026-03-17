use clock::Clock;

fn main() {
    let hours = 7;
    let minutes = 30;
    let clock = Clock::new(hours, minutes);
    println!("{}", clock);
}
