use crate::wheels::FrontTires;
use crate::wheels::RearTires;

mod wheels;


fn main() { //Hard coded the tires sizes into the main function
    let front_tires_mr2 = FrontTires {
        size: String::from("195/55"),
        brand: String::from("Falken"),
        model: String::from("Azenis"),
        mileage_installed: 7500,
        notes: String::from("Good, fast tires"),
    };

    let rear_tires_mr2 = RearTires {
        size: String::from("205/55"),
        brand: String::from("Falken"),
        model: String::from("Azenis"),
        mileage_installed: 7500,
        notes: String::from("Good, fast tires"),
    };

    //Add user input here to see what they want to know about there tires...
    println!("On the front of the car you are running {} {}, of size {}. You installed these tires at mileage {} and stated that they are {}. On the rear of the car, you are running {} {}, of size {}. You installed these tires at mileage {} and stated that they are {}. Anything else I can help you with today?", front_tires_mr2.brand, front_tires_mr2.model, front_tires_mr2.size, front_tires_mr2.mileage_installed, front_tires_mr2.notes, rear_tires_mr2.brand, rear_tires_mr2.model, rear_tires_mr2.size, rear_tires_mr2.mileage_installed,rear_tires_mr2.notes);
}



