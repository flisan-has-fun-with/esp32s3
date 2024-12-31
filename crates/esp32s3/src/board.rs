use std::{ops::DerefMut, sync::Arc};

use anyhow::{Context, Result};
use esp_idf_svc::hal::prelude::Peripherals;
use screen::EPaper;
use tokio::sync::Mutex;

pub mod screen;

#[derive(Clone)]
pub struct Board {
    pub(self) state: BoardState,
}

#[derive(Clone)]
pub(crate) struct BoardState {
    pub(self) e_paper: Arc<Mutex<EPaper>>,
}

impl Board {
    pub fn new() -> Result<Self> {
        log::info!("Board is being initialized.");

        // let per = Peripherals::take().unwrap();

        log::info!("Initializing screen...");
        let screen = EPaper::new().context("Failed to initialize the screen")?;

        Ok(Self {
            state: BoardState {
                e_paper: Arc::new(Mutex::new(screen)),
            },
        })
    }

    pub fn e_paper(&self) -> Arc<Mutex<EPaper>> {
        self.state.e_paper.clone()
    }
}
