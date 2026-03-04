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

    // Set bdu and if_inc, recomended for driver usage
    sensor.init_set(Init::DrvRdy).await.unwrap();

    // Select bus interface
    sensor
        .bus_mode_set(BusMode {
            interface: Interface::SelByHw,
            filter: Filter::Auto,
        })
        .await.unwrap();

    // Set Output Data rate
    let md = Md {
        odr: Odr::_200hz,
        avg: Avg::_4,
        ..Default::default()
    };

    sensor.mode_set(&md).await.unwrap();

    // Enable AH/QVAR function
    sensor.ah_qvar_en_set(1).await.unwrap();

    // Read samples in polling mode (no int)
    loop {
        // Read output only if new values are available
        if let Ok(all_sources) = sensor.all_sources_get().await {
            if all_sources.drdy_pres == 1 {
                if let Ok(data) = sensor.ah_qvar_data_get().await {
                    writeln!(tx, "QVAR [mV]: {:6.2} [LSB]: {}", data.mv, data.lsb).unwrap();
                }
            }
        }
    }
}
