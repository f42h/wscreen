use winapi::um::winuser::{
    GetSystemMetrics, 
    SM_CXSCREEN, 
    SM_CYSCREEN
};

pub enum ScreenLocations {
    TopLeft,
    TopRight,
    Center,
    BottomLeft,
    BottomRight
}
pub struct ApplicationSize {
    pub width: f32,
    pub height: f32,
    pub ignore_win_taskbar: bool
}

impl Default for ApplicationSize {
    fn default() -> Self {
        Self {
            width: 0.0,
            height: 0.0, 
            ignore_win_taskbar: false 
        }
    }
}

struct Axis {
    x: f32,
    y: f32
}

impl Axis {
    fn new() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

#[derive(Clone, Copy)]
pub struct WScreen {
    width: i32,
    height: i32
}

impl WScreen {
    pub fn new() -> Self {
        Self { 
            width: unsafe { GetSystemMetrics(SM_CXSCREEN) }, 
            height: unsafe { GetSystemMetrics(SM_CYSCREEN) }
        }
    }

    pub fn resolution(&self) -> (f32, f32) {
        (self.width as f32, self.height as f32)
    } 

    fn padding_tbar(&self, ignore_taskbar_space: bool) -> f32 {
        let taskbar_space = 91.0;

        if ignore_taskbar_space {
            0.0
        } else {
            taskbar_space
        }
    }

    pub fn coordinates_of(
        &self, 
        location: ScreenLocations, 
        app_size: Option<ApplicationSize>
    ) -> (f32, f32) {
        let mut pos = Axis::new();

        let pad_y = app_size.as_ref().map_or(
            pos.x, 
            |size| {
                self.padding_tbar(size.ignore_win_taskbar)
            }
        );
        
        match location {
            ScreenLocations::TopLeft => {
                (pos.x, pos.y)
            },
            ScreenLocations::TopRight => {
                pos.x = app_size.map_or(
                    (self.width - 1) as f32, 
                    |size| {
                        ((self.width - 1) as f32) - size.height
                    }
                );

                (pos.x, pos.y)
            },
            ScreenLocations::Center => {
                pos.x = app_size.as_ref().map_or(
                    (self.width / 2) as f32,
                    |size| {
                        (self.width as f32 - size.width) / 2.0
                    } 
                );
                pos.y = app_size.map_or(
                    (self.height / 2) as f32,
                    |size| {
                        (self.height as f32 - size.height) / 2.0
                    } 
                );

                (pos.x, pos.y)
            },
            ScreenLocations::BottomLeft => {
                pos.y = app_size.map_or(
                    (self.height - 1) as f32, 
                    |size| {
                        (self.height - 1) as f32 - (size.width + pad_y)
                    }
                );

                (pos.x, pos.y)
            },
            ScreenLocations::BottomRight => {
                pos.x = app_size.as_ref().map_or(
                    (self.width - 1) as f32, 
                    |size| {
                        ((self.width - 1) as f32) - size.width
                    }
                );
                pos.y = app_size.map_or(
                    (self.height - 1) as f32, 
                    |size| {
                        ((self.height - 1) as f32) - (size.width + pad_y)
                    }
                );

                (pos.x, pos.y)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn screen_resolution() {
        let res = WScreen::new();
        assert_eq!((1920.0, 1080.0), res.resolution())
    }

    #[test]
    fn specify_window_location_appsize() {
        let res = WScreen::new();
        let (pos_x, pos_y) = res.coordinates_of(
            ScreenLocations::TopRight, 
            // Take application width and height into account
            Some(ApplicationSize {
                width: 350.0,
                height: 370.0,
                ..Default::default()
            })
        );

        // Assume that resolution is 1920x1080
        assert_eq!(pos_x, 1549.0);
        assert_eq!(pos_y, 0.0);
    }

    #[test]
    fn specify_window_location() {
        let res = WScreen::new();
        let (pos_x, pos_y) = res.coordinates_of(
            ScreenLocations::BottomLeft, 
            None // Ignore application width and height
        );

        // Assume that resolution is 1920x1080
        assert_eq!(pos_x, 0.0);
        assert_eq!(pos_y, 1079.0);
    }
}
