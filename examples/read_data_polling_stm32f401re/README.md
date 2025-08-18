# ILPS22QS Pressure and Temperature Data Acquisition on STM32F401RE via I2C

This example demonstrates how to interface the **ILPS22QS** pressure sensor with an **STM32F401RE** microcontroller using the I2C bus. The program initializes the sensor, configures it for low-frequency pressure and temperature measurement, and outputs the sensor data over UART.

The code is written in Rust using the `stm32f4xx-hal` hardware abstraction layer and the `ilps22qs` sensor driver crate. It showcases sensor initialization, configuration, and polling-based data reading with serial output.

---

## Hardware Setup

- **Microcontroller Board:** STM32F401RE Nucleo-64
- **Sensor:** ILPS22QS pressure sensor (I2C interface)
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

- The microcontroller peripherals are initialized, including clocks, GPIO pins, I2C, and UART.
- The I2C bus is configured for 100 kHz Standard Mode using pins PB8 (SCL) and PB9 (SDA).
- UART is configured on PA2 at 115200 baud for serial output.
- A delay provider is initialized using the system timer.

### Sensor Setup

- The ILPS22QS sensor is initialized over I2C with a startup delay.
- The sensor's device ID is read and verified against the expected `ILPS22QS_ID`.
- The sensor is reset to default configuration and the program waits until reset completes.
- The AH/QVAR function is disabled to save power.
- Recommended settings are applied:
  - Block Data Update (BDU) and interface increment enabled.
  - Bus interface set to hardware-selected interface with automatic filter.
  - Output Data Rate (ODR) set to 4 Hz with 16-sample averaging and low-pass filter enabled.
  - Full scale set to 1260 hPa.

### Data Acquisition Loop

- The program continuously polls the sensor status register.
- When new pressure or temperature data is available (`drdy_pres` or `drdy_temp` flags set), it reads the sensor data.
- If AH_QVAR data is zero, it prints pressure in hPa and temperature in °C.
- Otherwise, it prints the AH_QVAR raw LSB value and temperature in °C.
- Data is output over UART for monitoring.

---

## Usage

1. Connect the ILPS22QS sensor to the STM32F401RE Nucleo board via I2C on pins PB8 (SCL) and PB9 (SDA).
2. Connect UART TX pin PA2 to a serial terminal interface.
3. Build and flash the Rust firmware onto the STM32F401RE.
4. Open a serial terminal at 115200 baud.
5. Observe pressure, temperature, and AH_QVAR data printed continuously.

---

## Notes

- This example uses polling to check for new sensor data.
- The AH/QVAR function is disabled to reduce power consumption.
- The sensor driver handles low-level register access and configuration.
- The environment is `#![no_std]` and `#![no_main]` for embedded Rust applications.
- Panic behavior is set to use ITM for debugging output (`panic_itm`).

---

## References

- [STM32F401RE Nucleo-64 Board](https://www.st.com/en/evaluation-tools/nucleo-f401re.html)
- [ILPS22QS Sensor Datasheet](https://www.st.com/resource/en/datasheet/ilps22qs.pdf)
- [stm32f4xx-hal Rust crate](https://docs.rs/stm32f4xx-hal)

---

*This README provides a detailed explanation of the embedded Rust program for pressure and temperature data acquisition on STM32F401RE using the ILPS22QS sensor via I2C with UART output.*