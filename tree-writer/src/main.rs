use spidev::{SpiModeFlags, Spidev, SpidevOptions, SpidevTransfer};
use std::{io, thread, time::Duration};
use tree_data_schema::{Renderers, FRAME_RATE};

// use mun_runtime::{invoke_fn, RuntimeBuilder};

mod renderers;
use crate::renderers::{red_wave, template, tree_canvas::TreeCanvas};

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

fn main() {
    let mut spi = create_spi().unwrap();

    let mut tick = 0;

    let renderer = Renderers::Template;

    loop {
        thread::sleep(Duration::from_millis(1000 / FRAME_RATE));

        let tree_canvas: TreeCanvas = match renderer {
            Renderers::RedWave => red_wave::draw(tick),
            Renderers::Template => template::draw(tick),
        };

        full_duplex(&mut spi, tree_canvas).unwrap();
        tick += 1;
    }
}
