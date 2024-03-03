/*
 * angle of between two vectors
 */
pub fn angle(a: &glm::Vec3, b: &glm::Vec3) -> f32 {
    glm::angle(a, b)
}

/*
 * rotate vector given an angle
 *
 * angle is in radians
 */
pub fn rotate2d(vec: &glm::Vec2, angle: f32) -> glm::Vec2 {
    let matrix = glm::mat2(angle.cos(), -angle.sin(), angle.sin(), angle.cos());

    matrix * vec
}

/*
 * returns the distance between 2 points in 2d space
 */
pub fn magnitude2d_squared(vec1: &glm::Vec2, vec2: &glm::Vec2) -> f32 {
    (vec1.x - vec2.x).powf(2.0) + (vec1.y - vec2.y).powf(2.0)
}
