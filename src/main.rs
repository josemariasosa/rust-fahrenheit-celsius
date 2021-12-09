use std::io;

#[derive(Debug)]
enum TempUnit {
    F,
    C
}

fn main() {
    let origin: (f32, TempUnit) = get_user_values();
    println!("User temperature: {} {:?}", origin.0, origin.1);
    //let destination: (f32, TempUnit) = convert_unit(origin, to_);
}

fn get_user_values() -> (f32, TempUnit) {
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
    return (value, unit);
}
