pub struct myRide {
    pub make: String,
    pub model: String,
    pub year: u16,
    pub trim: String,
}

// This implementation defines a struct called myRide which contains four fields:
// make: A String that holds the make of the vehicle.
// model: A String that holds the model of the vehicle.
// year: An unsigned 16-bit integer (u16) that holds the year of the vehicle.
// trim: A String that holds the trim level of the vehicle.
// You can create a new instance of myRide like this:

//below is an exampke of creating that struct called myRide

// let my_ride = myRide {
//     make: String::from("Honda"),
//     model: String::from("Civic"),
//     year: 2021,
//     trim: String::from("Sport"),
// };

// This creates a new myRide instance with the make "Honda", model "Civic", year 2021, and trim level "Sport". You can access the fields of the struct like this:


// println!("My ride is a {} {} {} {}.", my_ride.year, my_ride.make, my_ride.model, my_ride.trim);

// This will print out "My ride is a 2021 Honda Civic Sport.".

fn modify_my_ride(mut my_ride: myRide, new_make: String, new_model: String, new_year: u16, new_trim: String) -> myRide {
    my_ride.make = new_make;
    my_ride.model = new_model;
    my_ride.year = new_year;
    my_ride.trim = new_trim;
    return my_ride;
}

// This function takes in a mutable reference to a myRide struct, as well as four new values for the make, model, year, and trim fields. It then modifies the fields of the struct accordingly and returns the updated struct.

// Here's an example usage of this function:


//Below is an example of calling the function to change the values of myRide
// my_ride = modify_my_ride(my_ride, String::from("Toyota"), String::from("Corolla"), 2022, String::from("SE"));

println!("My new ride is a {} {} {} {}.", my_ride.year, my_ride.make, my_ride.model, my_ride.trim);

// This code will modify the myRide instance's values to represent a 2022 Toyota Corolla SE and print out "My new ride is a 2022 Toyota Corolla SE.".