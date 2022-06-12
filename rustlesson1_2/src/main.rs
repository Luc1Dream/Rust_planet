use std::io;
fn main() {
    let slide = String::from("This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal.");

    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    input.push_str(" ");
    let count = slide.matches(&input.as_str().trim()).count();
    print!("so luong {}",count);
    println!("{}",input);

}