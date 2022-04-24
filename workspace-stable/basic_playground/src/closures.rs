// https://www.youtube.com/watch?v=kZXJvLfjUS4
use std::thread;
use std::time::Duration;

struct Memo<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Memo<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Memo<T> {
        Memo {
            calculation,
            value: None,
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v: u32 = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

pub fn run() {
    let simulated_intensity: u32 = 10;
    let simulated_random_number: u32 = 7;

    generate_workout(simulated_intensity, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut memo_result = Memo::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", memo_result.value(intensity));
        println!("Next, do {} situps!", memo_result.value(intensity));
    } else if random_number == 3 {
        println!("Take a break today!")
    } else {
        println!("Today, run for {} minutes!", memo_result.value(intensity));
    }
}
