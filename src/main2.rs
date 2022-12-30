#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write,BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {

    //strings 
    /* 
    let mut st1 = String::new();
    st1.push('A');
    st1.push_str(" word");

    for word in st1.split_whitespace(){
        println!("{}",word);
    }
    
    let st2 = st1.replace("A", "Another");
    println!("{}",st2);
    */

    //more with strings
    /* 
    let str3 = String::from("x r t b h k k a m c");

    let mut v1: Vec<char> = str3.chars().collect();
    v1.sort();
    v1.dedup(); //delete duplicates

    for char in v1 {
        println!("{}", char);
    }

    let st4: &str = "Random String";
    let mut st5: String = st4.to_string();
    println!("{}", st5);

    let byte_arr1 = st5.as_bytes();
    let st6 = &st5[0..6];
    println!("String length : {}", st6.len());

    st5.clear();
    */

    //casting
    /* 
    let int_u8: u8 = 5;
    let int2_u8: u8 = 4;

    let int3_u32: u32 = (int_u8 as u32) + (int2_u8 as u32);
    */

    //enums
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday
    }

    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::Saturday | Day::Sunday => true,
                _ => false
            }
        }
    }

    let today:Day = Day::Tuesday;
    match today {
        Day::Monday => println!("Hate mondays"),
        Day::Tuesday => println!("Donut day"),
        Day::Wednesday => println!("Hump day"),
        Day::Thursday => println!("Pay day"),
        Day::Friday => println!("Almost weekend"),
        Day::Saturday => println!("Weekend"),
        Day::Sunday => println!("Weekend"),
    }

    println!("Is today the weekend {}", today.is_weekend())




}
