use std::io;

#[derive(Debug)]
#[derive(PartialEq)]
enum TempUnit {
    F,
    C
}

struct Temperature {
    value: f32,
    unit: TempUnit,
}

impl Temperature {
    pub fn get_value(&self) -> f32 {
        self.value
    }
    pub fn get_unit(&self) -> TempUnit {
        self.unit
    }
}

fn main() {
    let origin: Temperature = get_user_values();
    println!("User temperature: {} {:?}", origin.value, origin.unit);
    let to: TempUnit = swamp_temp_unit(origin.get_unit());
    let destination: Temperature = convert_unit(origin, to);
    println!("Result temperature: {} {:?}", destination.value, destination.unit);
}

fn calc_f2c(value: f32) -> f32 {
    (value - 32.0) * (5.0/9.0)
}

fn calc_c2f(value: f32) -> f32 {
    (value * (9.0/5.0)) + 32.0
}

fn convert_unit(origin: Temperature, to: TempUnit) -> Temperature {
    let from_to_pair: (TempUnit, TempUnit) = (origin.get_unit(), to);
    match from_to_pair {
        (TempUnit::F, TempUnit::C) => Temperature {
            value: calc_f2c(origin.get_value()),
            unit: TempUnit::C,
        },
        (TempUnit::C, TempUnit::F) => Temperature {
            value: calc_c2f(origin.get_value()),
            unit: TempUnit::F,
        },
        _ => origin,
    }
}

fn swamp_temp_unit(current_unit: TempUnit) -> TempUnit {
    match current_unit {
       TempUnit::C => TempUnit::F,
       TempUnit::F => TempUnit::C,
    }
}

fn get_user_values() -> Temperature {
    let unit: TempUnit = loop {
        let mut input = String::new();
        println!("Choose origin temperature unit (C/F):");
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        let u = input.trim().to_lowercase().chars().nth(0).unwrap();
        match u {
            'c' => break TempUnit::C,
            'f' => break TempUnit::F,
            _ => {
                println!("MUST be Celsius or Fahrenheit (C/F)");
                continue;
            },
        }
    };
    let value: f32 = loop {
        let mut input = String::new();
        println!("What is the temperature: ");
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => continue,
        }
    };
    Temperature { value: value, unit: unit }
}
