pub fn add(source: &String, destination: &String, size: &str) -> String {
    format!("    add{} {},{}\n", size, source, destination)
}

pub fn sub(source: &String, destination: &String, size: &str) -> String {
    format!("    sub{} {},{}\n", size, source, destination)
}