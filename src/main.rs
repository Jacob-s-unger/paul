#![allow(warnings)]

use crate::tires::FrontTires;
use crate::tires::RearTires;
use crate::tires::build_front_tire;
use crate::tires::build_rear_tire;


use crate::wheels::FrontWheels;
use crate::wheels::RearWheels;

use crate::engine::Block;

use crate::performance::PerformanceValues;



use std::io; //Input/Output module

mod tires;
mod wheels;
mod engine;
mod performance;


fn main() { 
    
    
    //Hard coded the tires sizes into the main function (Static for initial testng)
    let front_tires_mr2 = FrontTires {
        set: bool::from(true),
        size: String::from("195/55"),
        brand: String::from("Falken"),
        model: String::from("Azenis"),
        mileage_installed: 7500,
        notes: String::from("Good, fast tires"),
    };

    let rear_tires_mr2 = RearTires {
        set: bool::from(true),
        size: String::from("205/55"),
        brand: String::from("Falken"),
        model: String::from("Azenis"),
        mileage_installed: 7500,
        notes: String::from("Good, fast tires"),
    };

    //Add user input here to see what they want to know about there tires...
    //println!("On the front of the car you are running {} {}, of size {}. You installed these tires at mileage {} and stated that they are {}. On the rear of the car, you are running {} {}, of size {}. You installed these tires at mileage {} and stated that they are {}. Anything else I can help you with today?", front_tires_mr2.brand, front_tires_mr2.model, front_tires_mr2.size, front_tires_mr2.mileage_installed, front_tires_mr2.notes, rear_tires_mr2.brand, rear_tires_mr2.model, rear_tires_mr2.size, rear_tires_mr2.mileage_installed,rear_tires_mr2.notes);

    // let mut choice = String::new();
    // println!("Would you like to add front wheel data to your vehicle? (1 yes, 2 no)");
    // let b1 = std::io::stdin().read_line(&mut choice).unwrap();



// THis is a test function that loops.... need to replace this with some prompt

    println!("Please type something, or x to escape:");
    let mut input_string = String::new();

    while input_string.trim() != "x" {
        input_string.clear();
        io::stdin().read_line(&mut input_string).unwrap();
        println!("You wrote {}", input_string);
    }
    println!("See you later!"); 

}

 