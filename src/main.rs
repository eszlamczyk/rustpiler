use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut f = File::create("rust_test.s")?;
    
    add_fixsum_function(&mut f, "sex", 69)?;
    Ok(())
}


fn add_fixsum_function(
        file: &mut File, 
        function_name: &str, 
        number: u32) -> std::io::Result<()> {
    file.write(format!("  .global {}\n", function_name).as_bytes())?;
    file.write(format!("  .type {}, @function\n", function_name).as_bytes())?;
    file.write(format!("{}:\n", function_name).as_bytes())?;
    file.write(format!("  movl ${}, %eax\n", number).as_bytes())?;
    file.write(b"  ret\n")?;
    Ok(())
}