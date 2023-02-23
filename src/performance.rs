pub struct PerformanceValues {
    pub hrsprs: i64,
    pub torque: i64,
    pub zero_sixty: f64,
    pub quarter_mile: String,
    pub family: String,
}

impl Default for PerformanceValues {
    fn default() -> PerformanceValues {
        PerformanceValues{
            hrsprs: 69,
            torque: 420,
            zero_sixty: 2.5,
            quarter_mile: "How I live my Life".to_string(),
            family: "Always".to_string(),
        }
    }
}