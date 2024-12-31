mod bindings;
mod ffi;
pub mod font;
mod memory;

use std::convert::Infallible;

use anyhow::Result;
use bindings::{
    epd_clear, epd_draw_grayscale_image, epd_draw_hline, epd_full_screen, epd_init, epd_poweroff,
    epd_poweron, EPD_HEIGHT, EPD_WIDTH,
};
use embedded_graphics::{
    geometry::AnchorPoint,
    pixelcolor::{raw::ToBytes, Gray4},
    prelude::{Dimensions, DrawTarget, GrayColor, Point, Size},
    primitives::Rectangle,
    Pixel,
};
use esp_idf_svc::sys::esp_random;
use ffi::Framebuffer;

use crate::image::FullScreenImage;

pub struct EPaper {
    framebuffer: Framebuffer,
}

impl EPaper {
    pub const WIDTH: u32 = EPD_WIDTH;
    pub const HEIGHT: u32 = EPD_HEIGHT;

    pub fn new() -> Result<Self> {
        log::info!("Initializing framebuffer...");
        let framebuffer = Framebuffer::new()?;

        log::info!("Initializing the screen...");
        unsafe {
            epd_init();
        }
        log::info!("screen is initialized!");

        Ok(EPaper { framebuffer })
    }

    pub fn clear_then_draw(
        &mut self,
        mut op: impl FnMut(EPaperCanvas) -> Result<()>,
    ) -> Result<()> {
        op(EPaperCanvas { screen: self })?;
        unsafe {
            epd_poweron();
            epd_clear();
            self.draw_framebuffer();
            epd_poweroff();
        }

        Ok(())
    }

    pub fn draw(&mut self, mut op: impl FnMut(EPaperCanvas) -> Result<()>) -> Result<()> {
        op(EPaperCanvas { screen: self })?;
        unsafe {
            epd_poweron();
            self.draw_framebuffer();
            epd_poweroff();
        }

        Ok(())
    }

    fn draw_framebuffer(&mut self) {
        unsafe {
            epd_draw_grayscale_image(epd_full_screen(), self.framebuffer.to_point());
        }
    }
}

pub struct EPaperCanvas<'screen> {
    screen: &'screen mut EPaper,
}

impl EPaperCanvas<'_> {
    pub fn set_pixel(&mut self, x: usize, y: usize, color: u8) {
        if !(0..EPD_WIDTH).contains(&(x as u32)) {
            log::warn!("({}, {}) is out of bound (x).", x, y);
            return;
        }

        if !(0..EPD_HEIGHT).contains(&(y as u32)) {
            log::warn!("({}, {}) is out of bound (y).", x, y);
            return;
        }

        let (index, shift) = self.pixel_to_fb_index_and_shift(x, y);
        self.screen.framebuffer.0[index] = color << shift;
    }

    pub fn horizontal_line(&mut self, (x_start, x_end): (usize, usize), y: usize, color: u8) {
        if !(0..EPD_HEIGHT).contains(&(y as u32)) {
            log::warn!("({}, {}) is out of bound (y).", x_start, y);
            return;
        }

        if !(0..EPD_WIDTH).contains(&(x_start as u32)) {
            log::warn!("({}, {}) is out of bound (x).", x_start, y);
            return;
        }

        if !(0..EPD_WIDTH).contains(&(x_end as u32)) {
            log::warn!("({}, {}) is out of bound (x).", x_end, y);
            return;
        }

        let color = color & 0x0F;
        let color = color << 4 | color;

        let fill_start_x = x_start / 2 * 2 + 1;
        let fill_end_x = x_end / 2 * 2;

        let fill_start = self.pixel_to_fb_index_and_shift(fill_start_x, y).0;
        let fill_end = self.pixel_to_fb_index_and_shift(fill_end_x, y).0;

        self.screen.framebuffer.0[fill_start..=fill_end].fill(color);

        if x_start % 2 == 1 {
            self.set_pixel(x_start, y, color);
        }

        if x_end % 2 == 1 {
            self.set_pixel(x_end, y, color);
        }
    }

    pub fn rectangle(&mut self, (xs, ys): (usize, usize), (xe, ye): (usize, usize), color: u8) {
        for y in ys..=ye {
            self.horizontal_line((xs, xe), y, color);
        }
    }

    pub fn clear(&mut self) {
        unsafe {
            epd_poweron();
            epd_clear();
            epd_poweroff();
        }
    }

    pub async fn show_image(&mut self, image: &FullScreenImage) {
        self.screen.framebuffer.copy(image);
    }

    pub fn update(&mut self) {
        self.screen.draw_framebuffer();
    }

    fn pixel_to_fb_index_and_shift(&self, x: usize, y: usize) -> (usize, usize) {
        (
            (y * (EPD_WIDTH as usize) + x) / 2,
            if x % 2 == 0 { 4 } else { 0 },
        )
    }
}

impl DrawTarget for EPaperCanvas<'_> {
    type Color = Gray4;
    type Error = Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let size = self.bounding_box();

        pixels
            .into_iter()
            .filter(|Pixel(coord, _)| size.contains(*coord))
            .for_each(|Pixel(coord, color)| {
                self.set_pixel(coord.x as usize, coord.y as usize, color.luma());
            });

        Ok(())
    }

    fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) -> Result<(), Self::Error> {
        let area = area.intersection(&self.bounding_box());

        let top_left = area.top_left;
        let bottom_right = area.bottom_right().unwrap_or(top_left);

        self.rectangle(
            (top_left.x as usize, top_left.y as usize),
            (bottom_right.x as usize, bottom_right.y as usize),
            color.luma(),
        );

        Ok(())
    }
}

impl Dimensions for EPaperCanvas<'_> {
    fn bounding_box(&self) -> Rectangle {
        Rectangle::new(Point::new(0, 0), Size::new(960, 540))
    }
}
