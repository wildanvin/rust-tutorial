#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write,BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

use std::ops::Add;

use std::collections::HashMap;

fn get_sum_gen<T:Add<Output = T>>(x: T, y: T) -> T {
    return x + y;
}

fn main() {
    //generics:
    println!("5 + 4 = {}", get_sum_gen(5, 4) );
    println!("5.2 + 4.5 = {}", get_sum_gen(5.2, 4.5) );

    //hashmaps:
    let mut heroes = HashMap::new();
    heroes.insert("Superman", "Clark Kent");

    //structs:
    struct Customer {
        name: String,
        address: String,
        balance: f32,
    }

    let mut bob = Customer {
        name: String::from("Bob Smith"),
        address: String::from("555 Main Street"),
        balance: 238.98,
    };

    bob.address = String::from("505 Main St.");

    //struct with generics
    /* 
    struct Rectangle<T, U>{
        length: T,
        height: U,
    }

    let rec = Rectangle {length: 4, height: 10.5};
    */

    //traits
    const PI:f32 = 3.1416;
    trait Shape {
        fn new(length: f32, width: f32) -> Self;
        fn area(&self) -> f32;   
    }

    struct Rectangle {length: f32, width: f32};
    struct Circle {length: f32, width: f32};

    impl Shape for Rectangle {
        fn new(length: f32, width: f32) -> Rectangle {
            return Rectangle{length, width};
        }
        fn area(&self) -> f32 {
            return self.length * self.width;
        }
    }

    impl Shape for Circle {
        fn new(length: f32, width: f32) -> Circle {
            return Circle{length, width};
        }
        fn area(&self) -> f32 {
            return (self.length/2.0).powf(2.0)*PI;
        }
    }

    let rec: Rectangle = Shape::new(10.0,10.0);

    let circ: Circle = Shape::new(10.0,10.0);

    println!("Rec Area: {}", rec.area() );
    println!("Circ Area: {}", circ.area() );


}
