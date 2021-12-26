use log::info;
use spidev::{SpiModeFlags, Spidev, SpidevOptions, SpidevTransfer};
use std::{
    io,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
use tokio::task;
use tree_data_schema::{Renderers, FRAME_RATE};

mod renderers;
use crate::renderers::{
    ender_logo, mario, rainbow_wave, red_wave, snow, space_fight, template,
    tree_canvas::TreeCanvas, JWST,
};

// Set up the SPI interface
#[cfg(target_arch = "arm")]
fn create_spi() -> io::Result<Spidev> {
    let mut spi = Spidev::open("/dev/spidev0.0")?;
    let options = SpidevOptions::new()
        .bits_per_word(8)
        .max_speed_hz(10_000_000)
        .mode(SpiModeFlags::SPI_MODE_0)
        .build();
    spi.configure(&options)?;
    Ok(spi)
}

// Send the data to the SPI interface
#[cfg(target_arch = "arm")]
fn full_duplex(spi: &mut Spidev, tree_canvas: TreeCanvas) -> io::Result<()> {
    let mut rx_buf: [u8; 4500] = [0; 4500];
    let tx_buf = tree_canvas.convert_to_buffer();

    {
        let mut transfer = SpidevTransfer::read_write(&tx_buf, &mut rx_buf);
        spi.transfer(&mut transfer)?;
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::formatted_builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    #[cfg(target_arch = "arm")]
    let mut spi = create_spi().unwrap();

    let renderer = Arc::new(Mutex::new(Renderers::Snow));

    // This task will check the web server every second to see if there is a new
    // renderer
    let renderer_clone = renderer.clone();
    task::spawn(async move {
        let mut last_fail = false;
        loop {
            match reqwest::get("https://tree.dendropho.be/current_renderer").await {
                Ok(response) => {
                    if let Ok(body) = &response.text().await {
                        if let Ok(unwrapped_renderer) = serde_json::from_str::<Renderers>(body) {
                            let mut renderer = renderer_clone.lock().unwrap();
                            if *renderer != unwrapped_renderer {
                                *renderer = unwrapped_renderer;
                                info!("New renderer: {}", unwrapped_renderer);
                            }
                        }
                    }

                    // Reset last fail so we only show network errors once
                    last_fail = false;
                }
                Err(e) => {
                    if !last_fail {
                        info!("Failed to get renderer: {}", e);
                        last_fail = true;
                    }
                }
            };

            // Sleep for a second
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Loop forever, track the number of ticks that have elapsed
    let mut tick = 0;
    loop {
        thread::sleep(Duration::from_millis(1000 / FRAME_RATE));

        let renderer = *renderer.lock().unwrap();

        // Add your enum variant here (and remember to import it above)
        let tree_canvas: TreeCanvas = match renderer {
            Renderers::RedWave => red_wave::draw(tick),
            Renderers::Template => template::draw(tick),
            Renderers::Snow => snow::draw(tick),
            Renderers::EnderLogo => ender_logo::draw(tick),
            Renderers::SpaceFight => space_fight::draw(tick),
            Renderers::RainbowWave => rainbow_wave::draw(tick),
            Renderers::Mario => mario::draw(tick),
            Renderers::JWST => JWST::draw(tick),
        };

        #[cfg(target_arch = "arm")]
        full_duplex(&mut spi, tree_canvas).unwrap();

        tick += 1;
    }
}
