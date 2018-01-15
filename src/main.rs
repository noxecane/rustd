extern crate rustd;

use std::thread;
use std::time::Duration;

fn main() {
    let fake_intensity = 24;
    let fake_random_value = 8;

    generate_workout(fake_intensity, fake_random_value);
}


struct Memoizer<T: Fn(u32) -> u32> {
    calculation: T,
    value: Option<u32>,
}

impl<T> Memoizer<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Memoizer<T> {
        Memoizer {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, randomness: u32) {
    let mut mem = Memoizer::new(|num| {
        println!("In the toilet....");
        thread::sleep(Duration::from_secs(3));
        num
    });
    if intensity < 25 {
        println!("Do {} pushups today!", mem.value(intensity));
        println!("Do {} sitreps!", mem.value(intensity));
    } else {
        if randomness == 3 {
            println!("Take a break. Don't forget to get hydrated");
        } else {
            println!("Run for {} mins!",mem.value(intensity));
        }
    }
}
