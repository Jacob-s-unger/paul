pub struct fluid {
    set:bool, //1 for set, 0 for unset, used in catching if its been created by a create function.
    pub fluid_type: String,
    pub viscosity: String, //for the oil viscosity since it likely has a rating
    pub date_of_change: String,
    pub notes: String,
}

fn fetch_fluid(fluid_type: String, viscosity: String, date_of_change: String, notes:String) {
    println!("Fluid Type: {fluid_type}", fluid_type);
    println!("viscosity: {viscosity}", viscosity);
    println!("Date of Change on: {date_of_change}", {date_of_change});
    println!("Notes: {notes}", {notes})
}

fn change_fluids(fluid_type: String, viscosity: String, date_of_change: String, notes:String) -> fluid {
    fluid {
        fluid_type: fluid_type,
        viscosity: viscosity,
        date_of_change: date_of_change,
        notes: notes,
    }
}

fn check_fluid(liquid:fluid) { //This should check if fluids have been set and if they have, return the set fluid, otherwise prompts to set those fluid.
    if liquid.set == true {
        println!("You already have a fluid set. See below");
        fetch_fluid;
    } else {
        println!("No fluid has been set. If you would like to set a fluid, please call the change_fluid function");
    }
}

//Fluid Types...
// Diff fluid (Front and rear)
// Transmission Fluid
//Transfer case fluid
// Methanol (for those turbo stustus's!)
//Oil
//Breakfluid
//Coolant
//Powersteering fluid
//


// Will need a struct for each diffferent fluid until I learn how to dynamically create the struct with like check
// boxes for attributes that apply to the fluid...