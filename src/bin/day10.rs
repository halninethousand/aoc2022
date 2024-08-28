use std::collections::HashMap;

fn main() {
    let input: Vec<String> = include_str!("../data/day10.txt").lines().map(|s| s.to_owned()).collect();
    
    struct Machine {
        input: Vec<String>,
        cycle: u32,
        timer: HashMap<i32, u8>,
        register: i32,
        measure: [u32; 6],
        sum: i32,
    }

    impl Machine {
        fn new(input: Vec<String>) -> Self {
            Machine {
                input,
                cycle: 0,
                timer: HashMap::new(),
                register: 1,
                measure: [20, 60, 100, 140, 180, 220],
                sum: 0,
            }
        }

        fn r#loop(&mut self) {
            
            let mut ops = self.input.iter();

            loop {
                self.cycle += 1;

                if let Some(op) = ops.next() {
                    if op.starts_with("add") {
                        let mut op = op.split_whitespace();
                        op.next();
                        let value = op.next().unwrap().parse::<i32>().unwrap();
                        
                        self.timer.insert(value, 2);
                    }
                } else {
                    println!("EXHAUSTED");
                    if self.timer.is_empty() {
                        break;
                    }
                }
                
                for value in self.timer.values_mut() {
                    *value -= 1;
                }

                let keys_to_add: Vec<_> = self.timer 
                    .iter()
                    .filter(|&(_, &v)| v == 0)
                    .map(|(k, _)| k.clone())
                    .collect();

                let _: Vec<_> = keys_to_add
                    .iter()
                    .filter_map(|key| self.timer.remove(key))
                    .collect();

                for value in keys_to_add {
                    println!("adding VALUE {}", value);
                    self.register += value;
                }

                if self.measure.contains(&self.cycle) {
                    self.sum += self.cycle as i32 * self.register;
                    println!("CYCLE: {}, REGISTER: {}", self.cycle, self.register);
                }
            }
        }
    }

    let mut machine = Machine::new(input);
    machine.r#loop();
    println!("end sum: {:?}", machine.sum);
}
