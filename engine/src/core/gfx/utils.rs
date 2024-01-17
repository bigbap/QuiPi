pub fn normalise_dims(
    width: f32,
    height: f32,
    screen_width: f32,
    screen_height: f32
) -> (f32, f32) {
    (width / screen_width, height / screen_height)
}
