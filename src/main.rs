#![allow(warnings)]

use crate::tires::FrontTires;
use crate::tires::RearTires;
use crate::tires::build_front_tire;
use crate::tires::build_rear_tire;


use crate::wheels::FrontWheels;
use crate::wheels::RearWheels;

use crate::engine::Block;

use crate::performance::PerformanceValues;


use crate::myride::myRide; //Flagged for 0 usage in this file.... Seeing my garage does have implementations and is not flagged. THis can be resolved by actually calling a method from myRide here in main I think. In the final proposed structure, these use crates will not be necessary here I think because it will all be abstrated to their respective files, just calle don by main.
mod myride;

use crate::garage::myGarage;
mod garage;




use std::io; //Input/Output module

mod tires;
mod wheels;
mod engine;
mod performance;




// The main function will be a loop that either executes a menus command, goes back a menu, 
// or modifies/adds/deletes a value that is in a structure. Methods will be implemented to 
// be called upon when user selects that option from the menu via terminal for now, and 
// eventually presented with React Native as the front end

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


// THis is a test function that loops.... need to replace this with some prompt, see the information about the top of the main for the goals for this main function.

    println!("Please type something, or x to escape:");
    let mut input_string = String::new();

    while input_string.trim() != "x" {
        input_string.clear();
        io::stdin().read_line(&mut input_string).unwrap();
        println!("You wrote {}", input_string);
    }
    println!("See you later!"); 

}



//####################################################################//

//This is a method called myGarage that features functions for modifying the contents of my garage. This is a good wireframe for what the rest of the implementations will look like, however those implementations hsould be located in the files where the objects originate from for better abstractions and organization.

// impl myGarage {
//     // Method to add a new vehicle to the garage
//     fn add_vehicle(&mut self, vehicle: myRide) {
//         self.vehicles.push(vehicle);
//     }

//     // Method to remove a vehicle from the garage by its index
//     fn remove_vehicle(&mut self, index: usize) {
//         self.vehicles.remove(index);
//     }

//     // Method to get a reference to a vehicle by its index
//     fn get_vehicle(&self, index: usize) -> Option<&myRide> {
//         self.vehicles.get(index)
//     }

//     // Method to get the number of vehicles in the garage
//     fn num_vehicles(&self) -> usize {
//         self.vehicles.len()
//     }
// }

// fn main() {
//     // Create a new garage
//     let mut garage = myGarage { vehicles: vec![] };

//     // Add some vehicles to the garage
//     let car1 = myRide { make: "Toyota".to_string(), model: "Corolla".to_string(), year: 2018 };
//     let car2 = myRide { make: "Honda".to_string(), model: "Civic".to_string(), year: 2015 };
//     garage.add_vehicle(car1);
//     garage.add_vehicle(car2);

//     // Get the number of vehicles in the garage
//     println!("Number of vehicles in the garage: {}", garage.num_vehicles());

//     // Remove the first vehicle from the garage
//     garage.remove_vehicle(0);

//     // Get the remaining vehicle in the garage
//     if let Some(vehicle) = garage.get_vehicle(0) {
//         println!("Remaining vehicle: {} {} {}", vehicle.year, vehicle.make, vehicle.model);
//     }
// }
// This implementation uses a Vec to store the vehicles in the garage, 
//and provides methods to add, remove, and retrieve vehicles by their index. 
//You can add other methods as needed to track maintenance information for each vehicle.







 