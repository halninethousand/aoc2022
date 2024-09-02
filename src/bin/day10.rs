fn main() {
    let input: Vec<String> = include_str!("../data/day10.txt").lines().map(|s| s.to_owned()).collect();

    enum State {
        Busy(i32),
        Idle,
    }
    
    struct Machine {
        input: Vec<String>,
        cycle: u32,
        register: i32,
        measure: [u32; 6],
        sum: i32,
    }

    impl Machine {
        fn new(input: Vec<String>) -> Self {
            Machine {
                input,
                cycle: 0,
                register: 1,
                measure: [20, 60, 100, 140, 180, 220],
                sum: 0,
            }
        }

        fn tick(&mut self) {
            self.cycle += 1;

            if self.measure.contains(&self.cycle) {
                self.sum += self.cycle as i32 * self.register;
                println!("CYCLE: {}, REGISTER: {}", self.cycle, self.register);
            }
        }

        fn addx(&mut self, incr: i32) {
            self.tick();
            self.tick();
            self.register += incr;
        }

        fn run_instructions(&mut self) {
            for line in self.input.clone() {
                let instruction: Vec<&str> = line.split(' ').collect();

                match instruction[0] {
                    "noop" => self.tick(),
                    "addx" => self.addx(instruction[1].parse::<i32>().expect("Should be an int")),
                    _ => unimplemented!(),
                }
            }
        }
    }

    struct Machine_part2 {
        input: Vec<String>,
        row_length: i32,
        pixel_position: i32,
        register: i32,
        crt: String,
    }

    impl Machine_part2 {
        fn new(input: Vec<String>) -> Self {
            Machine_part2 {
                input,
                row_length: 40,
                pixel_position: 0,
                register: 1,
                crt: String::from(""),
            }
        }

        fn tick(&mut self) {
            self.crt.push(self.get_pixel());
            self.pixel_position += 1;
            if self.pixel_position == self.row_length {
                self.reset_pixel_position();
            }
        }

        fn addx(&mut self, incr: i32) {
            self.tick();
            self.tick();
            self.register += incr;
        }

        fn sprite_is_visible(&self) -> bool {
            (self.register - 1..=self.register + 1).contains(&self.pixel_position)
        }

        fn get_pixel(&self) -> char {
            if self.sprite_is_visible() {
                '#'
            } else {
                '.'
            }
        }

        fn reset_pixel_position(&mut self) {
            self.crt.push('\n');
            self.pixel_position = 0;
        }

        fn run_instructions(&mut self) {
            for line in self.input.clone() {
                let instruction: Vec<&str> = line.split(' ').collect();

                match instruction[0] {
                    "noop" => self.tick(),
                    "addx" => self.addx(instruction[1].parse::<i32>().expect("Should be an int")),
                    _ => unimplemented!(),
                }
            }
        }
    }

    let mut machine = Machine::new(input.clone());
    machine.run_instructions();
    println!("signal strength sum: {:?}", machine.sum);

    let mut machine_part2 = Machine_part2::new(input);
    machine_part2.run_instructions();
    print!("part 2 letters:\n{}", machine_part2.crt);
}
