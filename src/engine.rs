pub struct Block {
    set: bool, //1 for set, 0 for unset, used in catching if its been created by a create function.
    pub cylinder_count: String,
    pub cylinder_arrangment: String,
    pub engine_volume: f64,
}