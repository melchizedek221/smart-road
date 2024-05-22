pub fn calculate_distance(time: f32, velocity: f32) -> f32 {
    velocity * time
}

pub fn calculate_velocity(distance: f32, time: f32) -> f32 {
    distance / time
}