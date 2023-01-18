fn main() {
    let mut line = String::new();
    println!("Please enter your name: ");
    std::io::stdin().read_line(&mut line).unwrap();
    println!("{}", build_message(line));
}

fn build_message(name: String) -> String {
    let message = "You are welcome ".to_owned() + &name;
    return message;
}

#[test]
fn test_build_message(){
    let name = String::from("jj");

    assert_eq!(build_message(name), "You are welcome jj");
}