use std::time::Duration;

use embedded_graphics::{
    pixelcolor::Gray4,
    prelude::*,
    primitives::{PrimitiveStyle, PrimitiveStyleBuilder, StyledDrawable},
};
use esp32s3::{
    board::{screen::font::FontPlanner, Board},
    main,
    rect_ext::RectangleExt as _,
};
use tokio::time;
use u8g2_fonts::fonts::u8g2_font_f16_t_japanese1;

main! {
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
                "マルチクレートの世界から\nこんにちは!",
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
