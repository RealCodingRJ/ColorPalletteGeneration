use std::io::stdin;

fn get_color(color: &str) -> (){

    print!("{} ", u32::from_str_radix(color, 16).unwrap());
}

fn get_colors(color: u32) -> (){

    print!("{} ", u32::from_str_radix(color.to_string().as_str(), 16).unwrap());
}

fn generate_color(r: u32, g: u32) -> u32 {

    rand::random_range(r..g)
}

fn main() {
    let mut num = String::new();
    stdin().read_line(&mut num).unwrap();


    if num.starts_with("Rand")
    {
        let r_value = generate_color(1, 255);
        let g_value = generate_color(1, 255);
        let b_value = generate_color(1, 255);

        get_colors(r_value);
        get_colors(g_value);
        get_colors(b_value);
    }
    else
    {
        let number = num.trim();

        get_color(number);
        get_color(number);
        get_color(number);
    }

}
