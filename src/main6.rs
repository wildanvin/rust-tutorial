#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write,BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

use std::ops::Add;

use std::collections::HashMap;

//modules
mod restaurant;
use crate::restaurant::order_food;

fn main() {
   //modules
   order_food();


}
