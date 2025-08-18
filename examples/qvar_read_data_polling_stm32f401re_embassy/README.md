# ILPS22QS Sensor QVAR Data Acquisition on STM32F401RE with I2C Interface

This example demonstrates how to interface the **ILPS22QS** pressure sensor with an **STM32F401RE** microcontroller using the I2C bus to acquire QVAR (quasi-electrostatic potential variation) data. The program initializes the sensor, configures it for continuous pressure and QVAR sensing at 200 Hz output data rate, and outputs the QVAR data over UART for monitoring.

The code is written in Rust using the `embassy-stm32` async embedded framework and the `ilps22qs` sensor driver crate. It showcases sensor initialization, configuration, and polling-based data reading with blocking UART writes.

---

## Hardware Setup

- **Microcontroller Board:** STM32F401RE Nucleo-64
- **Sensor:** ILPS22QS pressure sensor with QVAR sensing channel
- **Communication Interface:** I2C1 at 100 kHz Standard Mode
- **UART:** USART2 for serial output at 115200 baud

### Default Pin Configuration

| Signal    | STM32F401RE Pin | Description                  |
|-----------|-----------------|------------------------------|
| I2C1_SCL  | PB8             | I2C clock line (open-drain)  |
| I2C1_SDA  | PB9             | I2C data line (open-drain)   |
| USART2_TX | PA2             | UART transmit for debug output|

The ILPS22QS sensor is connected to the STM32F401RE's I2C1 peripheral on pins PB8 (SCL) and PB9 (SDA). UART output is routed through PA2 for serial communication.

---

## Code Description

### Initialization

- The program initializes microcontroller peripherals using the `embassy-stm32` async runtime.
- UART is configured on PA2 at 115200 baud for serial output with blocking writes.
- The I2C bus is configured for 100 kHz Standard Mode using pins PB8 (SCL) and PB9 (SDA).
- A delay provider is used for sensor startup timing.

### Sensor Setup

- The ILPS22QS sensor is initialized over I2C with a startup delay.
- The sensor's device ID is read and verified against the expected `ILPS22QS_ID`.
- The sensor is reset to default configuration and the program waits until reset completes.
- Recommended settings are applied:
  - Block Data Update (BDU) and interface increment enabled.
  - Bus interface set to hardware-selected interface with automatic filter.
  - Output Data Rate (ODR) set to 200 Hz with 4-sample averaging.
  - AH/QVAR function enabled to provide quantized variance data.

### Data Acquisition Loop

- The program continuously polls the sensor status register.
- When new pressure data is available (`drdy_pres` flag set), it reads the QVAR data.
- The QVAR data is printed over UART in millivolts and raw LSB format using blocking writes.

---

## Usage

1. Connect the ILPS22QS sensor to the STM32F401RE Nucleo board via I2C on pins PB8 (SCL) and PB9 (SDA).
2. Connect UART TX pin PA2 to a serial terminal interface.
3. Build and flash the Rust firmware onto the STM32F401RE.
4. Open a serial terminal at 115200 baud.
5. Observe the QVAR sensor data printed continuously.

---

## Notes

- This example uses polling to check for new sensor data.
- UART writes are blocking; asynchronous UART with DMA is not used in this example.
- The QVAR sensing channel measures quasi-electrostatic potential variations, useful for user interface, water leak detection, and wire detection applications as described in the ST AN5755 application note.
- The sensor driver handles low-level register access and configuration.
- The environment is `#![no_std]` and `#![no_main]` for embedded Rust applications using the Embassy async runtime.
- Panic behavior is set to use `panic_probe`.

---

## References

- [STM32F401RE Nucleo-64 Board](https://www.st.com/en/evaluation-tools/nucleo-f401re.html)
- [ILPS22QS Sensor Datasheet](https://www.st.com/resource/en/datasheet/ilps22qs.pdf)
- [ST Application Note AN5755: QVAR Sensing Channel](https://www.st.com/resource/en/application_note/an5755-qvar-sensing-channel--stmicroelectronics.pdf)
- [embassy-stm32 Rust crate](https://docs.rs/embassy-stm32)

---

*This README provides a detailed explanation of the embedded Rust program for QVAR sensing on STM32F401RE using the ILPS22QS sensor via I2C with UART output, following STMicroelectronics guidelines.*