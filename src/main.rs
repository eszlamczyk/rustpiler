use std::fs::File;
use std::io::prelude::*;

mod instructions;

use crate::instructions::arithmetic::*;
use crate::instructions::basic::*;

fn main() -> std::io::Result<()> {

    let mut f = File::create("rust_test.s")?;
    
    add_fixsum_function(&mut f, "sex", 69)?;
    add_addx_function(&mut f, "add2", 2)?;
    add_removex_function(&mut f, "remove3", 3)?;
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

fn add_addx_function(        
    file: &mut File, 
    function_name: &str, 
    number: u32) -> std::io::Result<()> {
    file.write(format!("  .global {}\n", function_name).as_bytes())?;
    file.write(format!("  .type {}, @function\n", function_name).as_bytes())?;
    file.write(format!("{}:\n", function_name).as_bytes())?;
    file.write(push_stack().as_bytes())?;
    file.write(mov(&"%edi".to_string(), &"-0x4(%rbp)".to_string(), "l").as_bytes())?;
    file.write(add(&format!("$0x{}", number), &"-0x4(%rbp)".to_string(), "l").as_bytes())?;
    file.write(mov(&"-0x4(%rbp)".to_string(), &"%eax".to_string(), "l").as_bytes())?;
    file.write(pop_stack_and_return().as_bytes())?;
    Ok(())
}

fn add_removex_function(        
    file: &mut File, 
    function_name: &str, 
    number: u32) -> std::io::Result<()> {
    file.write(format!("  .global {}\n", function_name).as_bytes())?;
    file.write(format!("  .type {}, @function\n", function_name).as_bytes())?;
    file.write(format!("{}:\n", function_name).as_bytes())?;
    file.write(push_stack().as_bytes())?;
    file.write(mov(&"%edi".to_string(), &"-0x4(%rbp)".to_string(), "l").as_bytes())?;
    file.write(sub(&format!("$0x{}", number), &"-0x4(%rbp)".to_string(), "l").as_bytes())?;
    file.write(mov(&"-0x4(%rbp)".to_string(), &"%eax".to_string(), "l").as_bytes())?;
    file.write(pop_stack_and_return().as_bytes())?;
    Ok(())
}
