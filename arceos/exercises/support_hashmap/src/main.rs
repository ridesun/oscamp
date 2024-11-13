#![no_std]
#![no_main]

#[macro_use]
#[cfg(feature = "axstd")]
extern crate axstd as std;

use std::collections::HashMap;
#[cfg_attr(feature = "axstd", no_mangle)]
fn main() {
    println!("Running memory tests...");
    test_hashmap();
    test_hashmap_with_capacity();
    println!("Memory tests run OK!");
}

fn test_hashmap() {
    const N: u32 = 50_000;
    let start=std::time::Instant::now();
    let mut m = HashMap::new();
    for value in 0..N {
        let key = format!("key_{value}");
        m.insert(key, value);
    }
    for (k, v) in m.iter() {
        if let Some(k) = k.strip_prefix("key_") {
            assert_eq!(k.parse::<u32>().unwrap(), *v);
        }
    }
    let end=std::time::Instant::now();
    println!("test_hashmap() OK!");
    println!("time: {:.3}s",end.duration_since(start).as_secs_f32());
}
fn test_hashmap_with_capacity() {
    const N: u32 = 50_000;
    let start=std::time::Instant::now();
    let mut m = HashMap::with_capacity(N as usize);
    for value in 0..N {
        let key = format!("key_{value}");
        m.insert(key, value);
    }
    for (k, v) in m.iter() {
        if let Some(k) = k.strip_prefix("key_") {
            assert_eq!(k.parse::<u32>().unwrap(), *v);
        }
    }
    let end=std::time::Instant::now();
    println!("test_hashmap_with_capacity() OK!");
    println!("time: {:.3}s",end.duration_since(start).as_secs_f32());
}