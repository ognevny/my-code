// Russian poem

#![allow(unconditional_recursion)]
use std::{time::Duration, thread::sleep};

fn pop() -> ! {
    println!("У попа была собака,");
    sleep(Duration::from_secs_f32(1.5));
    println!("Он её любил.");
    sleep(Duration::from_secs_f32(1.5));
    println!("Она съела кусок мяса,");
    sleep(Duration::from_secs_f32(1.5));
    println!("Он её убил.");
    sleep(Duration::from_secs_f32(1.5));
    println!("В землю закопал,");
    sleep(Duration::from_secs_f32(1.5));
    println!("И написал он, что");
    sleep(Duration::from_secs_f32(1.5));
    pop() }

fn main() {
    pop() }
