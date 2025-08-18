#![no_main]
#![no_std]

use core::fmt::Write;

use ilps22qs_rs::prelude::*;
use ilps22qs_rs::{Ilps22qs, ILPS22QS_ID};

use panic_itm as _;

use cortex_m_rt::entry;
use stm32f4xx_hal::{
    i2c::{DutyCycle, I2c, Mode},
    pac,
    prelude::*,
    serial::Config,
};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(8.MHz()).sysclk(48.MHz()).freeze();

    let mut delay = cp.SYST.delay(&clocks);

    let gpiob = dp.GPIOB.split();
    let gpioa = dp.GPIOA.split();

    let scl = gpiob.pb8;
    let sda = gpiob.pb9;

    let i2c = I2c::new(
        dp.I2C1,
        (scl, sda),
        Mode::Fast {
            frequency: 400.kHz(),
            duty_cycle: DutyCycle::Ratio2to1,
        },
        &clocks,
    );

    let tx_pin = gpioa.pa2.into_alternate();
    let mut tx = dp
        .USART2
        .tx(
            tx_pin,
            Config::default()
                .baudrate(115200.bps())
                .wordlength_8()
                .parity_none(),
            &clocks,
        )
        .unwrap();

    delay.delay_ms(10);

    let mut sensor = Ilps22qs::new_i2c(i2c, ilps22qs_rs::I2CAddress::I2cAdd, delay);

    match sensor.id_get() {
        Ok(value) => {
            if value.whoami() != ILPS22QS_ID {
                panic!("Invalid sensors Id")
            }
        }
        Err(e) => writeln!(tx, "An error occured while reading sensor id: {:?}", e).unwrap(),
    }

    // Restore default configuration
    sensor.init_set(Init::Reset).unwrap();
    loop {
        if sensor.status_get().unwrap().sw_reset == 0 {
            break;
        }
    }

    // Disable AH/QVAR to save power consumption
    sensor.ah_qvar_disable().unwrap();
    // Set bdu and if_inc, recomended for driver usage
    sensor.init_set(Init::DrvRdy).unwrap();

    // Select bus interface
    sensor
        .bus_mode_set(BusMode {
            interface: Interface::SelByHw,
            filter: Filter::Auto,
        })
        .unwrap();

    // Set output Data rate
    let md = Md {
        interleaved_mode: 0,
        fs: Fs::_1260hpa,
        odr: Odr::_10hz,
        avg: Avg::_16,
        lpf: Lpf::OdrDiv4,
    };
    sensor.mode_set(&md).unwrap();

    // Enable FIFO
    sensor
        .fifo_mode_set(&FifoMd {
            operation: Operation::Stream,
            watermark: 32,
        })
        .unwrap();

    // Read samples in polling mode (no int)
    let mut data = [FifoData::default(); 32];
    loop {
        // Read output only if new values are available
        if let Ok(all_sources) = sensor.all_sources_get() {
            if all_sources.fifo_th > 0 {
                let level = sensor.fifo_level_get().unwrap_or(0);
                if let Ok(()) = sensor.fifo_data_get(level, &md, &mut data) {
                    writeln!(tx, "--- FIFO salmples").unwrap();
                    for i in 0..level {
                        if data[i as usize].lsb == 0 {
                            writeln!(tx, "{:02}: pressure [hPa]:{:6.2}", i, data[i as usize].hpa)
                                .unwrap();
                        } else {
                            writeln!(tx, "{:02}: AH_QVAR lsb: {}", i, data[i as usize].hpa)
                                .unwrap();
                        }
                    }
                    writeln!(tx).unwrap();
                }
            }
        }
    }
}
