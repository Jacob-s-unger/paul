pub fn build_front_wheel(size: String, brand: String, model: String, mileage_installed: usize, notes: String) -> FrontWheels {
    FrontWheels { 
        size: size,
        brand: brand,
        model: model,
        mileage_installed: mileage_installed,
        notes:notes,
     }
}

//Not currently used functions but will be used when user input is implemented.
pub fn build_rear_wheel(size: String, brand: String, model: String, mileage_installed: usize, notes: String) -> RearWheels {
    RearWheels { 
            size: size,
            brand: brand,
            model: model,
            mileage_installed: mileage_installed,
            notes:notes,
    }
}

pub struct FrontWheels {
    pub size: String,
    pub brand: String,
    pub model: String,
    pub mileage_installed: usize,
    pub notes: String,
}

pub struct RearWheels {
    pub size: String,
    pub brand: String,
    pub model: String,
    pub mileage_installed: usize,
    pub notes: String,
}
