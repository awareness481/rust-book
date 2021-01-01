use ::std::io;

fn main() {
    println!("Is your temperature in Fahrenheit or Celsius, F/C");

    let mut unit = String::new();

    io::stdin()
        .read_line(&mut unit)
        .expect("Failed to read line");

    let mut unit:String = match unit.trim().parse() {
        Ok(num) => num,
        Err(_) => unit,
    };

    let mut temp = String::new();

    println!("Input the value of the temperature");

    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

    let temp: f32 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => 1.0,
    };

    let mut result: f32 = 0.0;
    if unit.to_lowercase() == "f" {
        result = (temp - 32.00) / 1.8;
        unit = "C".to_string();
    } else if unit.to_lowercase() == "c" {
        result = temp * 1.8 + 32.00;
        unit = "F".to_string();
    }

    println!("Temperature is {} {}", result, unit);
}
