fn main() {
    let my_string = String::from("Elmo is a great developer!");
    let length = get_str_len(&my_string);
    println!("The length of the string is: {}", length);
}

fn get_str_len(str: &str) -> usize {
    str.chars().count()
}