pub fn add(source: &String, destination: &String, size: &str) -> String {
    format!("    add{size} {source},{destination}\n")
}

pub fn sub(source: &String, destination: &String, size: &str) -> String {
    format!("    sub{size} {source},{destination}\n")
}

pub fn inc(destination: &String, size: &str) -> String {
    format!("    inc{size} {destination}\n")
}

pub fn dec(destination: &String, size: &str) -> String {
    format!("    dec{size} {destination}\n")
}

pub fn neg(destination: &String, size: &str) -> String {
    format!("    neg{size} {destination}\n")
}

pub fn imul(source: &String, destination: &String, size: &str) -> String {
    format!("    imul{size} {source}, {destination}\n")
}

// 	%rax / S: quotient → %rax, remainder → %rdx
pub fn idiv(source: &String, size: &str) -> String {
    format!("    idiv{size} {source}\n")
}