use log::info;
use spidev::{SpiModeFlags, Spidev, SpidevOptions, SpidevTransfer};
use std::{io, thread, time::Duration};
use tree_data_schema::{Renderers, FRAME_RATE};

// use mun_runtime::{invoke_fn, RuntimeBuilder};

mod renderers;
use crate::renderers::{
    ender_logo, mario, rainbow_wave, red_wave, snow, space_fight, template,
    tree_canvas::TreeCanvas, JWST,
};

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

fn full_duplex(spi: &mut Spidev, tree_canvas: TreeCanvas) -> io::Result<()> {
    let mut rx_buf: [u8; 4500] = [0; 4500];
    let tx_buf = tree_canvas.convert_to_buffer();

    {
        let mut transfer = SpidevTransfer::read_write(&tx_buf, &mut rx_buf);
        spi.transfer(&mut transfer)?;
    }
    Ok(())
}

// TODO: Try this when we figure out Mun arrays
// fn load_from_mun() {
//     let mut runtime = RuntimeBuilder::new("../../tree-script/target/mod.munlib")
//         .spawn()
//         .expect("Failed to spawn Runtime");

//     runtime.borrow_mut().update();
//     let runtime_ref = runtime.borrow();

//     let tx_buf: i32 = invoke_fn!(runtime_ref, "build_array", tick).unwrap();
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::formatted_builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let mut spi = create_spi().unwrap();

    let mut tick = 0;

    let mut renderer = Renderers::Snow;

    let mut last_fail = false;

    loop {
        thread::sleep(Duration::from_millis(1000 / FRAME_RATE));

        // Make a request to get the current renderer
        // once a second
        if tick % (FRAME_RATE * 15) == 0 {
            renderer = match reqwest::get("https://tree.dendropho.be/current_renderer")
                .await
            {
                Ok(response) => {
                    let mut new_renderer = renderer;
                    if let Ok(body) = &response.text().await {
                        if let Ok(unwrapped_renderer) = serde_json::from_str::<Renderers>(body) {
                            new_renderer = unwrapped_renderer;
                        }
                    }

                    if new_renderer != renderer {
                        info!("Changing renderer to {}", new_renderer);
                    }

                    // Reset last fail so we only show network errors once
                    last_fail = false;
                    new_renderer
                }
                Err(e) => {
                    if !last_fail {
                        info!("Failed to get renderer: {}", e);
                        last_fail = true;
                    }
                    renderer
                }
            };
        }

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

        full_duplex(&mut spi, tree_canvas).unwrap();
        tick += 1;
    }
}
