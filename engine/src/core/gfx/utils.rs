pub fn normalise_dims_2d(
    x: f32,
    y: f32,
    screen_x: f32,
    screen_y: f32
) -> (f32, f32) {
    (x / screen_x, y / screen_y)
}
