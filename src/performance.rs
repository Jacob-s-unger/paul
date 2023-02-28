pub struct PerformanceValues {
    set: bool, //1 for set, 0 for unset, used in catching if its been created by a create function.
    pub hrsprs: i64,
    pub torque: i64,
    pub zero_sixty: f64,
    pub quarter_mile: String,
    pub family: String,
}

impl Default for PerformanceValues { //A potential default set that can be applied to a data structure.
    fn default() -> PerformanceValues {
        PerformanceValues{
            set: true,
            hrsprs: 69,
            torque: 420,
            zero_sixty: 2.5,
            quarter_mile: "How I live my Life".to_string(),
            family: "Always".to_string(),
        }
    }
}