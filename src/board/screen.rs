mod ffi;
mod bindings;
mod memory;

use anyhow::Result;
use bindings::{epd_clear, epd_draw_grayscale_image, epd_draw_hline, epd_full_screen, epd_init, epd_poweroff, epd_poweron, EPD_HEIGHT, EPD_WIDTH};
use esp_idf_svc::sys::esp_random;
use ffi::Framebuffer;

use crate::image::FullScreenImage;

pub struct Screen {
    framebuffer: Framebuffer,
}

impl Screen {
    pub fn new() -> Result<Self> {
        log::info!("Initializing framebuffer...");
        let framebuffer = Framebuffer::new()?;

        log::info!("Initializing the screen...");
        unsafe {
            epd_init();
            epd_poweron();
            epd_clear();
            epd_poweroff();
        }
        log::info!("screen is initialized!");

        Ok(Screen { framebuffer })
    }

    pub fn run_sample(&mut self) {
        unsafe {
            epd_poweron();
            epd_clear();
            for _ in 0..10 {
                epd_draw_hline(
                    10, (esp_random() % EPD_HEIGHT) as i32,
                    (EPD_WIDTH - 20) as i32,
                    0,
                    self.framebuffer.to_point()
                );
            }
            self.draw_framebuffer();
            epd_poweroff();
        };
    }

    pub fn clear(&mut self) {
        unsafe {
            epd_poweron();
            epd_clear();
            epd_poweroff();
        }
    }

    pub async fn show_image(&mut self, image: &FullScreenImage) {
        let epd_prep = tokio::spawn(async {
            log::info!("Initializing EPD for visualizing");
            unsafe {
                epd_poweron();
                epd_clear();
            }
            log::info!("EPD is initialized!");
        });
        log::info!("Copying image to framebuffer");
        self.framebuffer.copy(image);
        log::info!("Image is copied!");
        epd_prep.await;

        self.draw_framebuffer();
    }

    fn draw_framebuffer(&mut self) {
        unsafe {
            epd_draw_grayscale_image(epd_full_screen(), self.framebuffer.to_point());
        }
    }
}

