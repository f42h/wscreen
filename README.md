# Window Positioning Helper Structure for the Iced GUI Library

### Dependencies
- [winapi](https://crates.io/crates/winapi)

### Example
```rust
fn main() -> iced::Result {
    let resolution = WScreen::new();
    let (pos_x, pos_y) = resolution.coordinates_of(
        ScreenLocations::Center, 
        Some(ApplicationSize {
            width: 350.0,
            height: 370.0,
            ..Default::default()
        })
    );

    iced::application("MyApp", MyApp::update, MyApp::view)
        .window(Settings {
            min_size: Some(iced::Size::new(350.0, 370.0)),
            max_size: Some(iced::Size::new(400.0, 900.0)),
            size: iced::Size::new(350.0, 370.0),
            position: window::Position::Specific(Point::new(pos_x, pos_y)),
            ..Settings::default()
        })
        .run()
}
```