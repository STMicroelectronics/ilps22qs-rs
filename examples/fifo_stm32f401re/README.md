# ILPS22QS Pressure Sensor FIFO Data Acquisition on STM32F401RE Nucleo

This example demonstrates how to configure and read pressure data from the **ILPS22QS** pressure sensor using an **STM32F401RE** microcontroller board. The sensor is configured to stream pressure data into its FIFO buffer, which is read and processed by the microcontroller.

The program configures the sensor for 10 Hz output data rate with averaging and low-pass filtering, enables FIFO in stream mode with a watermark, and continuously polls the FIFO to read and print pressure samples over UART.

The code is written in Rust using the `stm32f4xx-hal` hardware abstraction layer and the `ilps22qs` sensor driver crate.

---

## Hardware Setup

- **Microcontroller Board:** STM32F401RE Nucleo-64
- **Sensor:** ILPS22QS Pressure Sensor
- **Communication Interface:** I2C1 at 100 kHz Standard Mode
- **UART:** USART2 for serial output at 115200 baud

### Default Pin Configuration

| Signal       | STM32F401RE Pin | Description                      |
|--------------|-----------------|---------------------------------|
| I2C1_SCL     | PB8             | I2C clock line (open-drain)     |
| I2C1_SDA     | PB9             | I2C data line (open-drain)      |
| USART2_TX    | PA2             | UART transmit for debug output  |

The ILPS22QS sensor is connected to the STM32F401RE via the I2C1 peripheral on pins PB8 (SCL) and PB9 (SDA). UART output is routed through PA2 for serial communication.

---

## Code Description

### Initialization

- The program initializes STM32F401RE peripherals including clocks, GPIO pins, I2C, and UART.
- I2C1 is configured for 100 kHz Standard Mode with open-drain pins PB8 and PB9.
- USART2 is configured on PA2 at 115200 baud for serial output.
- A delay abstraction is created using the system timer.

### Sensor Configuration

- The ILPS22QS sensor is initialized over I2C.
- The device ID is read and verified to confirm sensor presence.
- The sensor is reset to default configuration and waits until reset completes.
- The AH/QVAR feature is disabled to save power.
- Block Data Update (BDU) and interface increment are enabled for reliable driver operation.
- The bus interface is set to hardware-selected I2C with automatic filtering.
- The sensor is configured for:
  - Output data rate: 10 Hz
  - Full scale: 1260 hPa
  - Averaging: 16 samples
  - Low-pass filter: ODR divided by 4
- FIFO is enabled in stream mode with a watermark of 32 samples.

### Data Acquisition Loop

- The program continuously polls the sensor's status for FIFO watermark events.
- When the FIFO watermark is reached, it reads all available FIFO samples.
- Each sample is checked:
  - If `lsb` is zero, the sample is interpreted as pressure in hPa.
  - Otherwise, it is interpreted as AH/QVAR data (raw LSB).
- Samples are printed over UART.

---

## Usage

1. Connect the ILPS22QS sensor to the STM32F401RE Nucleo board via I2C on pins PB8 (SCL) and PB9 (SDA).
2. Build and flash the Rust firmware onto the STM32F401RE.
3. Open a serial terminal at 115200 baud on the USART2 TX pin (PA2).
4. Observe pressure samples and AH/QVAR data printed continuously over UART.

---

## Notes

- This example uses polling to read FIFO data without interrupts.
- The AH/QVAR feature is disabled to save power in this example.
- The environment is `#![no_std]` and `#![no_main]` for embedded Rust applications.
- Panic behavior is set to halt on panic (`panic_halt`).

---

## References
- [ILP22QS Datasheet](https://www.st.com/resource/en/datasheet/ilps22qs.pdf)
- [STM32F401RE Nucleo-64 Board](https://www.st.com/en/evaluation-tools/nucleo-f401re.html)
- [stm32f4xx-hal Rust crate](https://docs.rs/stm32f4xx-hal)

---

*This README provides a detailed explanation of the embedded Rust program for pressure sensor FIFO data acquisition on STM32F401RE using the ILPS22QS sensor.*