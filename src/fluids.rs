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