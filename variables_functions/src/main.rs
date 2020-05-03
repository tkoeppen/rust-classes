fn main() {
    // 2.1 Formatting ouput
    println!("Hello, {:?}...", ("John1", 1));

    // 2.2 Variables
    let mut name = "Rusty";
    println!("Welcome, {} to the Rust world!", name);
    name = "John2";
    println!("Welcome, {} to the Rust world!", name);

    let a = 1;
    let b = 2;
    let c: u32 = a + b;
    println!("c = {}", c);

    // 2.3 Function & ownership
    let names = vec!["Rusty".to_string(), "John3".to_string()];
    let new_names = greet(names);
    greet(new_names);

    let names = vec!["Rusty2".to_string(), "John4".to_string()];
    greet(names.clone());
    greet(names);
}

fn greet(mut names: Vec<String>) -> Vec<String> {
    println!("Welcome, {:?} to the Rust world!", names);
    names.clear();
    names.push("Jane".to_string());
    names
}
