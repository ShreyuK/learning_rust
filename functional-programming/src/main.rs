use std::thread;
use std::time::Duration;

fn main() {
    let simulated_intensity = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_intensity, simulated_random_number);
}

// A Cacher struct that caches the result of a closure
// to avoid recalculating it multiple times.
// The closure must take a u32 and return a u32.
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

// Implementation of the Cacher struct
// with methods to create a new Cacher and to get the cached value.
impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    // Creates a new Cacher with the provided closure.
    // The closure is stored in the Cacher and can be called later.
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    // Returns the cached value if it exists.
    // If it does not exist, it calculates the value using the closure,
    // caches it, and then returns it.
    // This method takes an argument of type u32,
    // which is passed to the closure for calculation.
    // The closure is expected to perform some expensive computation,
    // and the result is cached for future calls.
    // This allows the Cacher to avoid recalculating the value
    // if the same argument is provided again.
    // The closure is called only once for each unique argument,
    // ensuring efficiency in repeated calls.
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

fn generate_workout(intensity: u32, random_number: u32) {
    let mut cached_result = Cacher::new(|num| {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", cached_result.value(intensity));
        println!("Next, do {} situps!", cached_result.value(intensity));
    } else {
        if random_number == 7 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, do {} burpees!", cached_result.value(intensity));
        }
    }
}
