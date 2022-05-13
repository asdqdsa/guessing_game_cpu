use std::io;
// use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let mut max = 100;
    let mut min = 0;
    println!("INPUT<1-100>?");
    let mut state = String::new();
    io::stdin()
        .read_line(&mut state)
        .expect("ERROR: FAILED TO READ");
    let state = state
        .trim()
        .parse::<u32>()
        .expect("ERROR");

    // let mut cpu_guess :u32 = rand::thread_rng().gen_range(_min+1..=_max);
    let mut iter = 0;
    let mut cpu_guess = 50;
    loop {
        iter += 1;
        println!("CPU->?{}" ,cpu_guess);
        match state.cmp(&cpu_guess) {
            Ordering::Less => {
                println!("WRONG LESS != {}",cpu_guess);
                max = cpu_guess;
                cpu_guess = ((cpu_guess - min) / 2) + min;
            }
            Ordering::Greater => {
                println!("WRONG MORE != {}", cpu_guess);
                min = cpu_guess;
                cpu_guess = ((max - cpu_guess) / 2) + min;
            }
            Ordering::Equal => {
                println!("N:{} I:{}", cpu_guess, iter);
                break;
            }
        }
    }




}
