#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write,BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {

    //vectors:

    let vec1: Vec<i32> = Vec::new();
    let mut vec2 = vec![1,2,3,4];

    vec2.push(5);
    println!("1st : {}", vec2[0]);

    let second: &i32 = &vec2[1];
    
    match vec2.get(1){
        Some(second) => println!("2nd : {}", second),
        None => println!("No second value"),
        //What does some and none do?
    }

    for i in &mut vec2 {
        *i *= 2
        //how to know when to use & and &mut
    }

    for i in &vec2 {
        println!("{}", i);
    }

    println!("Vec Length {}", vec2.len());
    println!("Pop : {:?}", vec2.pop());
    // what does :? does?

}
