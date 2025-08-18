#![no_std]
#![no_main]

use core::fmt::Write;

use cortex_m::prelude::_embedded_hal_blocking_delay_DelayMs;
use embassy_executor::Spawner;
use embassy_stm32::i2c::I2c;
use embassy_stm32::time::khz;
use embassy_stm32::usart::{self, DataBits, Parity, UartTx};
use embassy_time::Delay;
use heapless::String;
use ilps22qs_rs::prelude::*;
use ilps22qs_rs::{Ilps22qs, ILPS22QS_ID};
use {defmt_rtt as _, panic_probe as _};

#[defmt::panic_handler]
fn panic() -> ! {
    core::panic!("panic via `defmt::panic!")
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    let mut usart_cfg = usart::Config::default();
    usart_cfg.baudrate = 115200;
    usart_cfg.data_bits = DataBits::DataBits8;
    usart_cfg.parity = Parity::ParityNone;

    let mut tx = UartTx::new_blocking(p.USART2, p.PA2, usart_cfg).unwrap();

    let i2c = I2c::new_blocking(p.I2C1, p.PB8, p.PB9, khz(100), Default::default());

    let mut delay = Delay;
    delay.delay_ms(10_u8);

    let mut sensor = Ilps22qs::new_i2c(i2c, ilps22qs_rs::I2CAddress::I2cAdd, delay);
    let mut msg: String<64> = String::new();

    match sensor.id_get() {
        Ok(value) => {
            if value.whoami() != ILPS22QS_ID {
                panic!("Invalid sensors Id")
            }
        }
        Err(e) => {
            writeln!(&mut msg, "Error reading sensor id: {:?}", e).unwrap();
            tx.blocking_write(msg.as_bytes()).unwrap();
            msg.clear();
        }
    }

    // Restore default configuration
    sensor.init_set(Init::Reset).unwrap();
    loop {
        if sensor.status_get().unwrap().sw_reset == 0 {
            break;
        }
    }

    // Set bdu and if_inc, recomended for driver usage
    sensor.init_set(Init::DrvRdy).unwrap();

    // Select bus interface
    sensor
        .bus_mode_set(BusMode {
            interface: Interface::SelByHw,
            filter: Filter::Auto,
        })
        .unwrap();

    // Set Output Data rate
    let md = Md {
        odr: Odr::_200hz,
        avg: Avg::_4,
        ..Default::default()
    };
    sensor.mode_set(&md).unwrap();

    // Enable AH/QVAR function
    sensor.ah_qvar_en_set(1).unwrap();

    let mut msg: String<64> = String::new();
    // Read samples in polling mode (no int)
    loop {
        // Read output only if new values are available
        if let Ok(all_sources) = sensor.all_sources_get() {
            if all_sources.drdy_pres == 1 {
                if let Ok(data) = sensor.ah_qvar_data_get() {
                    writeln!(&mut msg, "QVAR [mV]: {:6.2} [LSB]: {}", data.mv, data.lsb).unwrap();
                    tx.blocking_write(msg.as_bytes()).unwrap();
                    msg.clear();
                }
            }
        }
    }
}
