pub fn build_array(tick: u64) -> [u8; 4500] {
    let mut tx_buf: [u8; 4500] = [0; 4500];

    // Example of a full duplex transfer
    // Each loop is red, green, blue
    // for i in 0..tx_buf.len() / 3 {
    //     tx_buf[i * 3 + 0] = 0xFF;
    //     tx_buf[i * 3 + 1] = 0x00;
    //     tx_buf[i * 3 + 2] = 0x00;
    // }

    // Use the tick as a color value with a sin wave

    red_lines(&mut tx_buf, tick);

    tx_buf
}

fn red_lines(tx_buf: &mut [u8; 4500], tick: u64) {
    for i in 0..tx_buf.len() / 3 {
        let curr_tick = tick % 100;

        tx_buf[i * 3 + 0] = ((((tick + i as u64) as f32 * 0.1).sin() + 0.3) * 255.0) as u8;
        // ((tick % 100) as f32 * (i as f32 / tx_buf.len() as f32).sin() * 255.0) as u8;
        tx_buf[i * 3 + 1] = 0x00;
        tx_buf[i * 3 + 2] = 0x00;
    }
}
