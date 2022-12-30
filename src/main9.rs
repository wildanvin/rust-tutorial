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

//for concurrency
use std::thread;
use std::time::Duration;

use std::rc::Rc;
use std::cell::RefCell;
// Arc<T> provides shared ownership of a value
// Mutex blocks threads waiting for lock to be available
use std::sync::{Arc, Mutex};


fn main() {
    /* 
    // Create a thread with spawn
    thread::spawn(|| {
        for i in 1..25 {
            println!("Spawned thread : {}", i);
            // Forces thread to sleep and allow another thread to execute
            thread::sleep(Duration::from_millis(1));
        }
    });
    

    for i in 1..20 {
        println!("Main thread : {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    */


    // If we assign the return value for this thread to a variable
    // and then call join on it our program will wait for it to stop
    // executing
    let thread1 = thread::spawn(|| {
        for i in 1..25 {
            println!("Spawned thread : {}", i);
            // Forces thread to sleep and allow another thread to execute
            thread::sleep(Duration::from_millis(1));
        }
    });

    // There are no guarantees on when the threads will execute and
    // that they will complete execution
    for i in 1..20 {
        println!("Main thread : {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    // We call join here so that the main thread executes with thread1
    // unwrap handles the option Result which is Ok or Err
    thread1.join().unwrap();


    // ----- BANK ACCOUNT EXAMPLE -----

    // Bank struct just contains current balance
    pub struct Bank {
        balance: f32
    }

    // Allows for withdrawing money
    // Pass a mutable reference so bank can be used elsewhere
    // fn withdraw(the_bank: &mut Bank, amt: f32) {
    //         the_bank.balance -= amt;
    //     }


    // Create bank struct
    // let mut bank = Bank{balance: 100.00};
    // withdraw(&mut bank, 5.00);
    // println!("Balance : {}", bank.balance);

    // Create a customer thread that withdraws money
    // THIS WON'T WORK
    // fn customer(the_bank: &mut Bank){
    //     withdraw(the_bank, 5.00)
    // }

    // Can't do this closure may outlive the current function,
    // but it borrows `bank`, which is owned by the current function
    // If a thread can outlive the main function and the main function
    // has the bank which causes problems
    // thread::spawn(|| {
    //     customer(&mut bank)
    // }).join().unwrap();

    
   
    fn withdraw(the_bank: &Arc<Mutex<Bank>>, amt:f32){
        let mut bank_ref = the_bank.lock().unwrap();

        if bank_ref.balance < 5.00{
            println!("Current Balance : {} Withdrawal a smaller amount",
            bank_ref.balance);
        } else {
            bank_ref.balance -= amt;
            println!("Customer withdrew {} Current Balance {}",
            amt, bank_ref.balance);
        }
    }

    fn customer(the_bank: Arc<Mutex<Bank>>) {
        withdraw(&the_bank, 5.00);
    }

    let bank: Arc<Mutex<Bank>> =
      Arc::new(Mutex::new(Bank { balance: 20.00 }));

    // Creates 10 customer threads
    let handles = (0..10).map(|_| {

        // Clone duplicates an the bank object
        let bank_ref = bank.clone();
        thread::spawn(|| {
            customer(bank_ref)
        })
    });

    // Wait for all customers to finish
    for handle in handles {
        handle.join().unwrap();
    }

  println!("Total: {}", bank.lock().unwrap().balance);
   
}
