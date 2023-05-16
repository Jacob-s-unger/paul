pub fn build_front_tire(set: bool, size: String, brand: String, model: String, mileage_installed: usize, notes: String) -> FrontTires {
    FrontTires { 
        set: set, //true of false based on if the data has been set before. If created, set to true. Once deleted set to false.
        size: size,
        brand: brand,
        model: model,
        mileage_installed: mileage_installed,
        notes:notes,
     }
}

//Not currently used functions but will be used when user input is implemented.
pub fn build_rear_tire(set: bool, size: String, brand: String, model: String, mileage_installed: usize, notes: String) -> RearTires {
    RearTires { 
            set: set, //true of false based on if the data has been set before. If created, set to true. Once deleted set to false.
            size: size,
            brand: brand,
            model: model,
            mileage_installed: mileage_installed,
            notes:notes,
    }
}

// pub fn    FUNCTION FOR SETTING INFORMATION IN A STRUCTURE
// NEEDS TO HAVE A DEFAULT CASE FOR IF IT HASNT BEEN SET, RETURN MESSAGE STATING SO
//ADD FLAG TO STRUCTURES TO CATCH THE CASE OF IT BEING SET OR NOT.


//These require the "pub" word infront of both the structure and the attribute to make sure main can access them.
pub struct FrontTires {
    pub set: bool, //1 for set, 0 for unset, used in catching if its been created by a create function.
    pub size: String,
    pub brand: String,
    pub model: String,
    pub mileage_installed: usize,
    pub notes: String,
}

pub struct RearTires {
    pub set: bool, //1 for set, 0 for unset, used in catching if its been created by a create function.
    pub size: String,
    pub brand: String,
    pub model: String,
    pub mileage_installed: usize,
    pub notes: String,
}

