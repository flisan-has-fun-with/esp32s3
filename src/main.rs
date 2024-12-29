mod board;
mod image;

use std::{sync::{atomic::{AtomicUsize, Ordering}, Arc}, time::Duration};

use board::Board;
use image::FERRIS;
use tokio::time;

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

    let screen = board.screen();
    log::info!("Initialization success!");

    let mut screen = screen.lock().await;

    log::info!("Showing image...");
    screen.show_image(FERRIS).await;

    loop {
        time::sleep(Duration::from_secs(1)).await;
    }
}
