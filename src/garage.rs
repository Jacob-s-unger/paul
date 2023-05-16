// This is a structure for keeping myRides structures called myGarage

// use crate::myRide; //0 implementation warning... but this did solve the error associated with structure below

use crate::myride::myRide; //This instead of what is found above... ? 

pub struct myGarage { //
    vehicles: Vec<myRide>,
}


//This is the myRide Structure for reference
// pub struct myRide {
//     pub make: String,
//     pub model: String,
//     pub year: u16,
//     pub trim: String,
// }