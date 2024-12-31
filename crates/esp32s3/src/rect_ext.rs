use embedded_graphics::{
    prelude::{Point, Size},
    primitives::Rectangle,
};

pub trait RectangleExt {
    fn expand(&self, expand_by: i32) -> Self;
    fn shrink(&self, shrink_by: i32) -> Self;
}

impl RectangleExt for Rectangle {
    fn expand(&self, expand_by: i32) -> Self {
        let (width, height) = if expand_by > 0 {
            (
                self.size.width.saturating_add(expand_by as u32 * 2),
                self.size.height.saturating_add(expand_by as u32 * 2),
            )
        } else {
            (
                self.size.width.saturating_sub(-expand_by as u32 * 2),
                self.size.height.saturating_sub(-expand_by as u32 * 2),
            )
        };

        Rectangle {
            top_left: self.top_left.move_by(-expand_by, -expand_by),
            size: Size { width, height },
        }
    }

    fn shrink(&self, shrink_by: i32) -> Self {
        self.expand(-shrink_by)
    }
}

pub trait PointExt {
    fn move_by(&self, x: i32, y: i32) -> Self;
}

impl PointExt for Point {
    fn move_by(&self, x: i32, y: i32) -> Self {
        Point {
            x: self.x.saturating_add(x),
            y: self.y.saturating_add(y),
        }
    }
}
