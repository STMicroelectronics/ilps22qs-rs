# ilps22qs-rs
[![Crates.io][crates-badge]][crates-url]
[![BSD 3-Clause licensed][bsd-badge]][bsd-url]

[crates-badge]: https://img.shields.io/crates/v/ilps22qs-rs
[crates-url]: https://crates.io/crates/ilps22qs-rs
[bsd-badge]: https://img.shields.io/crates/l/ilps22qs-rs
[bsd-url]: https://opensource.org/licenses/BSD-3-Clause

This crate provides a platform-agnostic driver for the ST ILPS22QS pressure and temperature sensor.

## Sensor Overview

The ILPS22QS is an ultracompact piezoresistive absolute pressure sensor that functions as a digital output barometer, supporting dual full-scale up to user-selectable 4060 hPa. The device delivers ultralow pressure noise with very low power consumption and operates over an extended temperature range from -40°C to +105°C.

The device comprises a sensing element and an IC interface which communicates over I²C, MIPI I3CSM, or SPI interfaces from the sensing element to the application and supports 1.2 V digital interface as well. The sensing element, which detects absolute pressure, consists of a suspended membrane manufactured using a dedicated process developed by ST.

The ILPS22QS embeds an analog hub sensing functionality, which is able to connect an analog input and convert it to a digital signal for embedded processing. In addition, an embedded Qvar (electric charge variation detection) channel can be enabled for sensing in applications such as water leakage detection, tap, double tap, long press, and L/R - R/L swipe.

The ILPS22QS is available in a full-mold, holed LGA package (HLGA). The package is holed to allow external pressure to reach the sensing element.

For more info, please visit the device page at [https://www.st.com/en/mems-and-sensors/ilps22qs.html](https://www.st.com/en/mems-and-sensors/ilps22qs.html).


## Installation

Add the driver to your `Cargo.toml` dependencies:

```toml
[dependencies]
ilps22qs-rs = "2.0.0"
```

Or, add it directly from the terminal:

```sh
cargo add ilps22qs-rs
```

## Usage

By default, the create exposes the **asynchronous** API, and it could be included using:
```rust
use ilps22qs_rs::asynchronous as ilps22qs;
use ilps22qs::*;
use ilps22qs::prelude::*;
```

### Blocking API (optional feature)

To use the **blocking** API instead of the asynchronous one, disable default features and enable the `blocking` feature in your Cargo.toml
```toml
[dependencies]
ilps22qs_rs-rs = { version = "2.0.0", default-features = false, features = ["blocking"] }
```
or from the terminal:
```sh
cargo add ilps22qs_rs-rs --no-default-features --features blocking
```

Then import the blocking API:
```rust
use ilps22qs_rs::blocking as ilps22qs;
use ilps22qs::*;
use ilps22qs::prelude::*;
```

### Create an instance

Create an instance of the driver with the `new_<bus>` associated function, by passing an I2C (`embedded_hal::i2c::I2c`) instance and I2C address, or an SPI (`embedded_hal::spi::SpiDevice`) instance, along with a timing peripheral.

An example with I2C:

```rust
let mut sensor = Ilps22qs::new_i2c(i2c, I2CAddress::I2cAdd, delay);
```

### Check "Who Am I" Register

This step ensures correct communication with the sensor. It returns a unique ID to verify the sensor's identity.

```rust
let value = sensor.id_get().unwrap();
if value.whoami != ID {
    panic!("Invalid sensor ID");
}
```

### Configure

See details in specific examples; the following are common api calls:

```rust
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

// Set Output Data rate
let md = Md {
    interleaved_mode: 0,
    fs: Fs::_1260hpa,
    odr: Odr::_4hz,
    avg: Avg::_16,
    lpf: Lpf::OdrDiv4,
};
sensor.mode_set(&md).unwrap();
```

## License

Distributed under the BSD-3 Clause license.

More Information: [http://www.st.com](http://st.com/MEMS).

**Copyright (C) 2025 STMicroelectronics**