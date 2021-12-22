use spidev::{SpiModeFlags, Spidev, SpidevOptions, SpidevTransfer};
use std::{io, thread, time::Duration};
use tree_data_schema::{Renderers, FRAME_RATE};
use tree_writer::renderers::TreeCanvas;

// use mun_runtime::{invoke_fn, RuntimeBuilder};

mod renderers;
use renderers::build_array;

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

    {
        let mut transfer =
            SpidevTransfer::read_write(&tree_canvas.convert_to_buffer(), &mut rx_buf);
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

    let renderer = Renderers::RedLines;

    loop {
        thread::sleep(Duration::from_millis(1000 / FRAME_RATE));

        let tree_canvas: TreeCanvas = match renderer {
            Renderers::RedLines => build_array(tick),
        };

        full_duplex(&mut spi, tree_canvas).unwrap();
        tick += 1;
    }
}
