use std::io::stdin;

fn get_color(color: &str) -> (){

    print!("{} ", u32::from_str_radix(color, 16).unwrap());
}

fn main() {

    let mut num = String::new();
    stdin().read_line(&mut num).unwrap();

    let number_hex: &str = num.trim();

    get_color(number_hex);
    get_color(number_hex);
    get_color(number_hex);
}