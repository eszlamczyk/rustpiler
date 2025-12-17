pub fn sal(destination: &String, value: &String, size: &str) -> String {
    format!("    sal{size} {value}, {destination}\n")
}

pub fn shl(destination: &String, value: &String, size: &str) -> String {
    format!("    shl{size} {value}, {destination}\n")
}

pub fn sar(destination: &String, value: &String, size: &str) -> String {
    format!("    sar{size} {value}, {destination}\n")
}

pub fn shr(destination: &String, value: &String, size: &str) -> String {
    format!("    shr{size} {value}, {destination}\n")
}