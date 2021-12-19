#[derive(Clone)]
pub struct Observation {
    pub channel_id: String,
    pub value: f64,
    pub timestamp: i32,
}
