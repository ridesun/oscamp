#![cfg_attr(feature = "axstd", no_std)]
#![cfg_attr(feature = "axstd", no_main)]

#[cfg(feature = "axstd")]
use axstd::println_color;
use axstd::println;

#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    println!("[WithColor]: Hello, Arceos!");
    println_color!("#9BE15D", "#00E3AE","[WithColor]: Hello, Arceos!");
    println_color!("#FFFF00", "#EE82EE","[WithColor]: Hello, Arceos!");
    println_color!("#F83600", "#F9D423","[WithColor]: Hello, Arceos!");
}
