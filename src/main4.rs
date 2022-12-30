#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write,BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

//functions
fn say_hello(){
    println!("Hello");
}

fn get_sum(x: i32, y: i32){
    println!("{} + {} = {}", x,y,x+y)
}

fn get_sum_2(x: i32, y: i32) -> i32 {
    x + y
}

fn get_2 (x: i32) -> (i32, i32){
    return (x+1, x+2);
}

fn sum_list(list: &[i32]) -> i32 {
    let mut sum = 0;
    for &val in list.iter(){
        sum += &val;
    }
    sum
}

fn main() {
    say_hello();
    get_sum(3, 4);
    println!("{}", get_sum_2(3, 8));

    //
    let (val_1, val_2) = get_2(3);
    println!("{} and {}", val_1, val_2);

    //
    let num_list = vec![1,2,3,4,5];
    println!("Sum of list = {}", sum_list(&num_list));

    

}
