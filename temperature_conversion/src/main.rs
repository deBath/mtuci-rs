use std::io;

fn main() {
    let mut f_temp_str = String::new();

    println!("Введите температуру в градусах по Фаренгейту:");
    match io::stdin().read_line(&mut f_temp_str) {
        Ok(_) => {},
        Err(e) => println!("ОШИБКА ВВОДА - {}", e)
    }

    let f_temp: f64 = f_temp_str.trim().parse().unwrap();

    let c_temp = (f_temp - 32.0) / 1.8;

    println!("Температура в градусах по Цельсию = {}", c_temp);
}