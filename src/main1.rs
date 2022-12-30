#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write,BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    
    //arrays:
    /* 
    let arr_1 = [1,2,3,4];
    println!("1st : {}", arr_1[0]);
    println!("length : {}", arr_1.len());
    */

    // loop in an array:
    /* 
    let mut loop_idx = 0;
    loop {
        if arr_1[loop_idx]%2 == 0 {
            loop_idx += 1;
            continue;
        }
        if arr_1[loop_idx] == 3 {
            break;
        }
        println!("Val : {}",arr_1[loop_idx]);
        loop_idx += 1;

    }
    */

    //with while loop:
    /* 
    let arr_2 = [1,2,3,4,5,6,7,8,9];
    let mut loop_idx = 0;

    while loop_idx < arr_2.len(){
        println!("Arr : {}", arr_2[loop_idx]);
        loop_idx += 1;

    }
    */

    // with for loops:
    /* 
    let arr_2 = [1,2,3,4,5,6,7,8,9];
    let mut loop_idx = 0;

    for val in arr_2.iter(){
        println!("Val : {}", val)
    }
    */

    // tuples: can contain differetn data types
    let my_tuple: (u8, String, f64) = (29, "Wilman".to_string(), 50_000.00);

    println!("Name : {}", my_tuple.1);
    let(v1, v2, v3) = my_tuple;

    println!("Age : {}", v1);






}
