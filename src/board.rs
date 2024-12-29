use std::{ops::DerefMut, sync::Arc};

use anyhow::{Context, Result};
use esp_idf_svc::hal::prelude::Peripherals;
use screen::Screen;
use tokio::sync::Mutex;

pub mod screen;

#[derive(Clone)]
pub struct Board {
    pub(self) state: BoardState,
}

#[derive(Clone)]
pub(crate) struct BoardState {
    pub(self) screen: Arc<Mutex<Screen>>,
}

impl Board {
    pub fn new() -> Result<Self> {
        log::info!("Board is being initialized.");
    
        // let per = Peripherals::take().unwrap();

        log::info!("Initializing screen...");
        let screen = Screen::new()
            .context("Failed to initialize the screen")?;

        Ok(Self {
            state: BoardState {
                screen: Arc::new(Mutex::new(screen)),
            }
        })
    }

    pub fn screen(&self) -> Arc<Mutex<Screen>> {
        self.state.screen.clone()
    }
}

