use defmt::info;
use maybe_async::maybe_async;
use crate::*;

#[maybe_async]
pub async fn run<B, D, L>(bus: B, mut tx: L, mut delay: D, _irq: ()) -> !
where
    B: BusOperation,
    D: DelayNs + Clone,
    L: embedded_io::Write
{
    use ilps22qs::prelude::*;
    use ilps22qs::*;

    info!("Configuring the sensor");
    let mut sensor = Ilps22qs::from_bus(bus, delay.clone());

    // boot time
    delay.delay_ms(10).await;

    // Check device ID
    let id = sensor.id_get().await.unwrap().whoami();
    info!("Device ID: {:x}", id);
    if id != ILPS22QS_ID {
        info!("Unexpected device ID: {:x}", id);
        writeln!(tx, "Unexpected device ID: {:x}", id).unwrap();
        loop {}
    }

    // Restore default configuration
    sensor.init_set(Init::Reset).await.unwrap();
    loop {
        if sensor.status_get().await.unwrap().sw_reset == 0 {
            break;
        }
    }

    // Disable AH/QVAR to save power consumption
    sensor.ah_qvar_disable().await.unwrap();
    // Set bdu and if_inc, recomended for driver usage
    sensor.init_set(Init::DrvRdy).await.unwrap();

    // Select bus interface
    sensor
        .bus_mode_set(BusMode {
            interface: Interface::SelByHw,
            filter: Filter::Auto,
        })
        .await.unwrap();

    // Set output Data rate
    let md = Md {
        interleaved_mode: 0,
        fs: Fs::_1260hpa,
        odr: Odr::_10hz,
        avg: Avg::_16,
        lpf: Lpf::OdrDiv4,
    };
    sensor.mode_set(&md).await.unwrap();

    // Enable FIFO
    sensor
        .fifo_mode_set(&FifoMd {
            operation: Operation::Stream,
            watermark: 32,
        })
        .await.unwrap();

    // Read samples in polling mode (no int)
    let mut data = [FifoData::default(); 32];
    loop {
        // Read output only if new values are available
        if let Ok(all_sources) = sensor.all_sources_get().await {
            if all_sources.fifo_th > 0 {
                let level = sensor.fifo_level_get().await.unwrap_or(0);
                if let Ok(()) = sensor.fifo_data_get(level, &md, &mut data).await {
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
