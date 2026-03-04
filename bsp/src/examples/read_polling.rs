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

    // Set Output Data rate
    let md = Md {
        interleaved_mode: 0,
        fs: Fs::_1260hpa,
        odr: Odr::_4hz,
        avg: Avg::_16,
        lpf: Lpf::OdrDiv4,
    };
    sensor.mode_set(&md).await.unwrap();

    // Read samples in polling mode (no int)
    loop {
        // Read output only if new values are available
        if let Ok(all_sources) = sensor.all_sources_get().await {
            if all_sources.drdy_pres == 1 || all_sources.drdy_temp == 1 {
                if let Ok(data) = sensor.data_get(&md).await {
                    if data.ah_qvar.lsb == 0 {
                        writeln!(
                            tx,
                            "Pressure [hPa]: {:6.2} Temperature [degC]: {:6.2}",
                            data.pressure.hpa, data.heat.deg_c
                        )
                        .unwrap();
                    } else {
                        writeln!(
                            tx,
                            "AH_QVAR lsb: {} Temperature [degC]: {:6.2}",
                            data.ah_qvar.lsb, data.heat.deg_c
                        )
                        .unwrap();
                    }
                }
            }
        }
    }
}
