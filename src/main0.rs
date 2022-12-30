#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write,BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    // Get input from the user and greet him:
    /*
    println!("What is your name?");
    
    let mut name = String::new();
    let greeting="Nice to meet you";
    io::stdin().read_line(&mut name)
    .expect("Didn't Receive Input");

    println!("Hello {}! {}", name.trim_end(), greeting);
    */
    /**************************************************** */

    //More about data types and casting 
    //You can do shadowing in Rust
    /*
    const ONE_MIL: u32= 1_000_000;
    const PI: f32 = 3.141592;
    let age ="47";
    let mut age: u32 = age.trim().parse().expect("Age wasn't assigned a number");
    age = age + 1;
    println!("I'm {} and I want ${}", age, ONE_MIL
    */

    /**************************************************** */
    // See varios max values for number types
    // boolean, chars, simple math

    /* 

    println!("Max u32: {}", u32::MAX);
    println!("Max u64: {}", u64::MAX);
    println!("Max usize: {}", usize::MAX);
    println!("Max u128: {}", u128::MAX);
    println!("Max f32: {}", f32::MAX);
    println!("Max f64: {}", f64::MAX);

    let is_true = true;
    let my_grade = 'A';

    let num_1: f32 = 1.111111111111111;
    println!("f32 : {}", num_1 + 0.111111111111111);

    let num_2: f64 = 1.111111111111111;
    println!("f64 : {}", num_2 + 0.111111111111111);
    */

    /**************************************************** */
    // if statements
    let age = 18;
    if (age >= 1) && (age <= 18){
        println!("Important Birthday");
    }else if (age == 21) || (age == 50){
        println!("Important Birthday");  
    }else if age >= 64{
        println!("Important Birthday");
    }else {
        println!("NOT Important Birthday");
    }

    //ternary operator ish
    let mut my_age = 17;
    let can_vote = if my_age >= 18 {
        true
    } else {
        false
    };
    println!("Can vote : {}", can_vote);

    //match
    let age2 = 8;
    match age2 {
        
        1..=18 => println!("Important Birthday"),
        21 | 50 => println!("Important Birthday"),
        65..=i32::MAX => println!("Important Birthday"),
        _ => println!("NOT Important Birthday"),
    };

    //more advanced match
    let my_age_3 = 18;
    let voting_age = 18;

    match my_age_3.cmp(&voting_age){
        Ordering::Less => println!("Cant vote"),
        Ordering::Equal => println!("You gained the right to vote"),
        Ordering::Greater => println!("Can vote"),

    };


}
