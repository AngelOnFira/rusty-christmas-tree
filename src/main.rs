use spidev::{SpiModeFlags, Spidev, SpidevOptions, SpidevTransfer};
use std::{io, thread, time::Duration};

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

fn full_duplex(spi: &mut Spidev, tick: i32) -> io::Result<()> {
    let mut tx_buf: [u8; 4500] = [0; 4500];
    let mut rx_buf: [u8; 4500] = [0; 4500];

    // Example of a full duplex transfer
    // Each loop is red, green, blue
    // for i in 0..tx_buf.len() / 3 {
    //     tx_buf[i * 3 + 0] = 0xFF;
    //     tx_buf[i * 3 + 1] = 0x00;
    //     tx_buf[i * 3 + 2] = 0x00;
    // }

    // Use the tick as a color value with a sin wave
    for i in 0..tx_buf.len() / 3 {
        tx_buf[i * 3 + 0] = (tick as f32 * (i as f32 / tx_buf.len() as f32).sin() * 255.0) as u8;
        tx_buf[i * 3 + 1] = 0x00;
        tx_buf[i * 3 + 2] = 0x00;
    }

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
        thread::sleep(Duration::from_millis(300));
        full_duplex(&mut spi, tick).unwrap();
        tick += 1;
    }
}
