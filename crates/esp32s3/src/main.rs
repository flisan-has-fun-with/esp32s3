mod board;
mod image;
pub mod rect_ext;

use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

use board::{
    screen::{font::FontPlanner, EPaper},
    Board,
};
use embedded_graphics::{
    mono_font::{
        ascii::FONT_9X18_BOLD, iso_8859_10::FONT_9X18, jis_x0201::FONT_10X20, MonoTextStyle,
        MonoTextStyleBuilder,
    },
    pixelcolor::Gray4,
    prelude::*,
    primitives::{PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, StyledDrawable},
    text::{Alignment, Text},
};
use image::{BOARD, TEST_PATTERNS};
use rect_ext::RectangleExt;
use tokio::time;
use u8g2_fonts::{
    fonts::u8g2_font_f16_t_japanese1,
    types::{FontColor, HorizontalAlignment, VerticalPosition},
    FontRenderer,
};

async fn future(au: Arc<AtomicUsize>) {
    loop {
        let prev = au.fetch_add(1, Ordering::SeqCst);
        log::info!("Counter: {}", prev + 1);

        time::sleep(Duration::from_secs(1)).await;
    }
}

fn main() {
    // https://github.com/aedm/esp32-s3-rust-axum-example/tree/main
    //
    //

    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    esp_idf_svc::io::vfs::initialize_eventfd(1).expect("Failed to initialize eventfd");

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Failed to build Tokio runtime");

    rt.block_on(async { _main().await });
}

async fn _main() {
    log::info!("Hello, world!");
    let board = Board::new().unwrap();

    let e_paper = board.e_paper();
    log::info!("Initialization success!");

    let mut e_paper = e_paper.lock().await;
    e_paper
        .clear_then_draw(|mut canvas| {
            let screen = canvas.bounding_box();

            screen
                .shrink(20)
                .draw_styled(
                    &PrimitiveStyleBuilder::new()
                        .stroke_color(Gray4::new(0x0))
                        .stroke_width(2)
                        .fill_color(Gray4::new(0xD))
                        .build(),
                    &mut canvas,
                )?;

            let plan = FontPlanner::at_center::<u8g2_font_f16_t_japanese1>(
                "こんにちは世界!",
                &screen,
            ).0.issueless().unwrap();

            plan
                .ensured_rendered_box()
                .expand(10)
                .draw_styled(&PrimitiveStyle::with_fill(Gray4::new(0xf)), &mut canvas)?;

            plan.0.draw(Gray4::new(0x0), &mut canvas)?;

            Ok(())
        })
        .unwrap();

    loop {
        time::sleep(Duration::from_secs(1)).await;
    }
}
