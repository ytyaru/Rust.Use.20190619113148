enum Color {
    Red,
    Green,
    Blue,
}
fn main() {
    use Color::{Red,Blue};
    let red = Red;
    let blue = Blue;
    let green = Color::Green;
}

