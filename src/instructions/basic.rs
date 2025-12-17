pub fn mov(source: &String, destination: &String, size: &str) -> String {
    format!("    mov{size} {source},{destination}\n")
}

pub fn push_stack() -> String {
    "    push %rbp
    mov %rsp, %rbp\n".to_string()
}

pub fn pop_stack_and_return() -> String {
    "    pop %rbp
    retq\n".to_string()
}