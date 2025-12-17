pub fn push(source: &String) -> String {
    format!("    push {source}\n")
}

pub fn pop(destination: &String) -> String {
    format!("    pop {destination}\n")
}