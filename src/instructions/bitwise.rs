pub fn and(source: &String, destination: &String, size: &str) -> String {
    format!("    and{size} {source}, {destination}\n")
}

pub fn or(source: &String, destination: &String, size: &str) -> String {
    format!("    or{size} {source}, {destination}\n")
}

pub fn xor(source: &String, destination: &String, size: &str) -> String {
    format!("    xor{size} {source}, {destination}\n")
}

pub fn not(destination: &String, size: &str) -> String {
    format!("    not{size} {destination}\n")
}