use aidan_tree::FRAME_RATE;
use spidev::{SpiModeFlags, Spidev, SpidevOptions, SpidevTransfer};
use std::{io, thread, time::Duration};

mod render;

use render::build_array;

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

fn full_duplex(spi: &mut Spidev, tx_buf: [u8; 4500]) -> io::Result<()> {
    let mut rx_buf: [u8; 4500] = [0; 4500];

    {
        let mut transfer = SpidevTransfer::read_write(&tx_buf, &mut rx_buf);
        spi.transfer(&mut transfer)?;
    }
    Ok(())
}

fn main() {
    let mut spi = create_spi().unwrap();

    let mut tick = 0;

    loop {
        thread::sleep(Duration::from_millis(1000 / FRAME_RATE));
        let tx_buf = build_array(tick);
        full_duplex(&mut spi, tx_buf).unwrap();
        tick += 1;
    }
}
