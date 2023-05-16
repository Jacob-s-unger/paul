pub struct Trans {
    set: bool, //1 for set, 0 for unset, used in catching if its been created by a create function.
    pub trans_model: String,
    pub auto_manual: bool, //0 for auto, 1 for manual. Will get translated in func
    pub gears: String,
    pub fluid: String, //This will prlly get moved to fluids section.
}

//Implement the methods here eventually to report on self stuct I think