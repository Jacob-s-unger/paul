pub fn build_front_tire(size: String, brand: String, model: String, mileage_installed: usize, notes: String) -> FrontTires {
    FrontTires { 
        size: size,
        brand: brand,
        model: model,
        mileage_installed: mileage_installed,
        notes:notes,
     }
}

//Not currently used functions but will be used when user input is implemented.
pub fn build_rear_tire(size: String, brand: String, model: String, mileage_installed: usize, notes: String) -> RearTires {
    RearTires { 
            size: size,
            brand: brand,
            model: model,
            mileage_installed: mileage_installed,
            notes:notes,
    }
}

//These require the "pub" word infront of both the structure and the attribute to make sure main can access them.
pub struct FrontTires {
    pub size: String,
    pub brand: String,
    pub model: String,
    pub mileage_installed: usize,
    pub notes: String,
}

pub struct RearTires {
    pub size: String,
    pub brand: String,
    pub model: String,
    pub mileage_installed: usize,
    pub notes: String,
}
