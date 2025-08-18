use crate::{BusOperation, Error, Ilps22qs};
use bitfield_struct::bitfield;
use derive_more::TryFrom;
use embedded_hal::delay::DelayNs;
use st_mem_bank_macro::register;

/// Represents the register addresses for device configuration and data retrieval.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum Reg {
    /// Address for the interrupt configuration register.
    InterruptCfg = 0x0B,
    /// Address for the low threshold register.
    ThsPL = 0x0C,
    /// Address for the high threshold register.
    ThsPH = 0x0D,
    /// Address for the interface control register.
    IfCtrl = 0x0E,
    /// Address for the device identification register.
    WhoAmI = 0x0F,
    /// Address for the control register 1.
    CtrlReg1 = 0x10,
    /// Address for the control register 2.
    CtrlReg2 = 0x11,
    /// Address for the control register 3.
    CtrlReg3 = 0x12,
    /// Address for the FIFO control register.
    FifoCtrl = 0x14,
    /// Address for the FIFO watermark register.
    FifoWtm = 0x15,
    /// Address for the low reference register.
    RefPL = 0x16,
    /// Address for the high reference register.
    RefPH = 0x17,
    /// Address for the I3C interface control register.
    I3cIfCtrl = 0x19,
    /// Address for the low pressure offset register.
    RpdsL = 0x1A,
    /// Address for the high pressure offset register.
    RpdsH = 0x1B,
    /// Address for the interrupt source register.
    IntSource = 0x24,
    /// Address for the FIFO status register 1.
    FifoStatus1 = 0x25,
    /// Address for the FIFO status register 2.
    FifoStatus2 = 0x26,
    /// Address for the status register.
    Status = 0x27,
    /// Address for the pressure output XL register.
    PressOutXl = 0x28,
    /// Address for the pressure output L register.
    PressOutL = 0x29,
    /// Address for the pressure output H register.
    PressOutH = 0x2A,
    /// Address for the temperature output L register.
    TempOutL = 0x2B,
    /// Address for the temperature output H register.
    TempOutH = 0x2C,
    /// Address for disabling the analog hub.
    AnalogicHubDisable = 0x5F,
    /// Address for the FIFO data output pressure XL register.
    FifoDataOutPressXl = 0x78,
    /// Address for the FIFO data output pressure L register.
    FifoDataOutPressL = 0x79,
    /// Address for the FIFO data output pressure H register.
    FifoDataOutPressH = 0x7A,
}

/// Interrupt mode configuration register.
///
/// Configuration options:
/// * `phe` (1 bit): Enable interrupt generation on pressure high event.
/// * `ple` (1 bit): Enable interrupt generation on pressure low event.
/// * `lir` (1 bit): Latch interrupt request to the [`Reg1::IntSource`] register.
/// * `reset_az` (1 bit): Reset AUTOZERO function.
/// * `autozero` (1 bit): Enable AUTOZERO function.
/// * `reset_arp` (1 bit): Reset AUTOREFP function.
/// * `autorefp` (1 bit): Enable AUTOREFP function.
///
/// # Bit Order
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::InterruptCfg, access_type = Ilps22qs, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct InterruptCfg {
    #[bits(1)]
    pub phe: u8,
    #[bits(1)]
    pub ple: u8,
    #[bits(1)]
    pub lir: u8,
    #[bits(1, access = RO)]
    not_used_01: u8,
    #[bits(1)]
    pub reset_az: u8,
    #[bits(1)]
    pub autozero: u8,
    #[bits(1)]
    pub reset_arp: u8,
    #[bits(1)]
    pub autorefp: u8,
}

/// User-defined threshold value for pressure interrupt event.
///
/// # Configuration options:
/// * `ths`: This register contains the threshold value for pressure interrupt
/// generation.
///
/// # Bit Order
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::ThsPL, access_type = Ilps22qs, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u16, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u16, order = Lsb))]
pub struct ThsP {
    #[bits(15)]
    pub ths: u16,
    #[bits(1, access = RO)]
    not_used_01: u8,
}

/// Interface control register.
///
/// # Configuration options:
/// * `cs_pu_dis` (1 bit): Disable pull-up on CS pin.
/// * `sda_pu_en` (1 bit): Enable pull-up on the SDA pin.
/// * `en_spi_read` (1 bit): Enable SPI read mode. This bit must be set to 1 before using the
/// 3-wire SPI interface.
/// * `i2c_i3c_dis` (1 bit): Disable I2C and I3C digital interfaces.
///
/// # Bit Order
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::IfCtrl, access_type = Ilps22qs, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IfCtrl {
    #[bits(1, access = RO)]
    not_used_01: u8,
    #[bits(1)]
    pub cs_pu_dis: u8,
    #[bits(2, access = RO)]
    not_used_02: u8,
    #[bits(1)]
    pub sda_pu_en: u8,
    #[bits(1)]
    pub en_spi_read: u8,
    #[bits(1)]
    pub i2c_i3c_dis: u8,
    #[bits(1, access = RO)]
    not_used_03: u8,
}

/// Control register 1.
///
/// Configuration options:
/// * `avg` (3 bits): Average selection.
/// * `odr` (4 bits): Output data rate selection.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::CtrlReg1, access_type = Ilps22qs, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct CtrlReg1 {
    #[bits(3)]
    pub avg: u8,
    #[bits(4)]
    pub odr: u8,
    #[bits(1, access = RO)]
    not_used_01: u8,
}

/// Control register 2.
///
/// Configuration options:
/// * `oneshot` (1 bit): Enables one-shot mode.
/// * `swreset` (1 bit): Software reset. The bit is self-cleared when the reset is completed.
/// * `bdu` (1 bit): Block data update.
/// * `en_lpfp` (1 bit): Enables low-pass filter on pressure data.
/// * `lfpf_cfg` (1 bit): Low-pass filter configuration.
/// * `fs_mode` (1 bit): Full-scale selection.
/// * `boot` (1 bit): Reboots memory content.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::CtrlReg2, access_type = Ilps22qs, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct CtrlReg2 {
    #[bits(1)]
    pub oneshot: u8,
    #[bits(1, access = RO)]
    not_used_01: u8,
    #[bits(1)]
    pub swreset: u8,
    #[bits(1)]
    pub bdu: u8,
    #[bits(1)]
    pub en_lpfp: u8,
    #[bits(1)]
    pub lfpf_cfg: u8,
    #[bits(1)]
    pub fs_mode: u8,
    #[bits(1)]
    pub boot: u8,
}

/// Control register 3.
///
/// Configuration options:
/// * `if_add_inc` (1 bit): Register address automatically incremented during a multiple byte
/// access with a serial interface (I2C or SPI).
/// * `ah_qvar_p_auto_en` (1 bit): Enables AH/Qvar and pressure hardware interleaved mode.
/// * `ah_qvar_en` (1 bit): Enables AH (analog hub)/Qvar functions.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::CtrlReg3, access_type = Ilps22qs, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct CtrlReg3 {
    #[bits(1)]
    pub if_add_inc: u8,
    #[bits(4, access = RO)]
    not_used_01: u8,
    #[bits(1)]
    pub ah_qvar_p_auto_en: u8,
    #[bits(1, access = RO)]
    not_used_02: u8,
    #[bits(1)]
    pub ah_qvar_en: u8,
}

/// FIFO control register.
///
/// Configuration options:
/// * `f_mode` (2 bits): Selects triggered FIFO modes.
/// * `trig_modes` (1 bit): Enables triggered FIFO modes.
/// * `stop_on_wtm` (1 bit): Stop-on-FIFO watermark. Enables FIFO watermark level use.
/// * `ah_qvar_p_fifo_en` (1 bit): Enables AH/Qvar and pressure hardware interleaved mode in FIFO
/// buffer.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::FifoCtrl, access_type = Ilps22qs, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FifoCtrl {
    #[bits(2)]
    pub f_mode: u8,
    #[bits(1)]
    pub trig_modes: u8,
    #[bits(1)]
    pub stop_on_wtm: u8,
    #[bits(1)]
    pub ah_qvar_p_fifo_en: u8,
    #[bits(3, access = RO)]
    not_used_01: u8,
}

/// FIFO threshold setting register.
///
/// Configuration options:
/// * `wtm` (7 bits): FIFO threshold. Watermark level setting.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::FifoWtm, access_type = Ilps22qs, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FifoWtm {
    #[bits(7)]
    pub wtm: u8,
    #[bits(1, access = RO)]
    not_used_01: u8,
}

/// Reference pressure data.
///
/// Configuration options:
/// `refp`: This register contains the reference pressure value.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::RefPL, access_type = Ilps22qs, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u16, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u16, order = Lsb))]
pub struct RefP {
    #[bits(16, access = RO)]
    pub refp: u16,
}

/// Control register.
///
/// Configuration options:
/// * `asf_on` (1 bits): Enable anti-spike filters.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::I3cIfCtrl, access_type = Ilps22qs, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct I3cIfCtrl {
    #[bits(5, access = RO)]
    not_used_02: u8,
    #[bits(1)]
    pub asf_on: u8,
    #[bits(2, access = RO)]
    not_used_01: u8,
}

/// Pressure offset register.
///
/// This register contains the offset pressure value used for one-point calibration (OPC) after soldering.
/// The offset value is a 16-bit signed integer expressed in two's complement format. It is composed of
/// the RPDS_H and RPDS_L registers and is added directly to the compensated pressure data to improve accuracy.
///
/// # Fields
///
/// * `rpds` - The pressure offset calibration value as a 16-bit signed integer (read-only).
#[register(address = Reg::RpdsL, access_type = Ilps22qs, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u16, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u16, order = Lsb))]
pub struct Rpds {
    #[bits(16, access = RO)]
    pub rpds: i16,
}

/// Interrupt source (read only) register for differential pressure. A read at this address clear
/// the register itself.
///
/// Configuration options:
/// * `ph` (1 bit): Differential pressure High.
/// * `pl` (1 bit): Differential pressure Low.
/// * `ia` (1 bit): Interrupt active.
/// * `boot_on` (1 bit): Indication that Boot (reboot) phase is running.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::IntSource, access_type = Ilps22qs, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct IntSource {
    #[bits(1, access = RO)]
    pub ph: u8,
    #[bits(1, access = RO)]
    pub pl: u8,
    #[bits(1, access = RO)]
    pub ia: u8,
    #[bits(4, access = RO)]
    not_used_01: u8,
    #[bits(1, access = RO)]
    pub boot_on: u8,
}

/// FIFO status register (read only).
///
/// Configuration options:
/// * `fss` (8 bits): FIFO stored data level, number of unread samples stored in FIFO.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::FifoStatus1, access_type = Ilps22qs, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FifoStatus1 {
    #[bits(8, access = RO)]
    pub fss: u8,
}

/// FIFO status register (read only).
///
/// Configuration options:
/// * `fifo_full_ia` (1 bit): FIFO full status.
/// * `fifo_ovr_ia` (1 bit): FIFO overrun status.
/// * `fifo_wtm_ia` (1 bit): FIFO threshold (watermark) status.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::FifoStatus2, access_type = Ilps22qs, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct FifoStatus2 {
    #[bits(5, access = RO)]
    not_used_01: u8,
    #[bits(1, access = RO)]
    pub fifo_full_ia: u8,
    #[bits(1, access = RO)]
    pub fifo_ovr_ia: u8,
    #[bits(1, access = RO)]
    pub fifo_wtm_ia: u8,
}

/// Status register (read only).
///
/// This register is updated every ODR cycle.
///
/// Configuration options:
/// * `p_da` (1 bit): Pressure data available.
/// * `t_da` (1 bit): Temperature data available.
/// * `p_or` (1 bit): Pressure data overrun.
/// * `t_or` (1 bit): Temperature data overrun.
///
/// The bit order for this struct can be configured using the `bit_order_msb` feature:
/// * `Msb`: Most significant bit first.
/// * `Lsb`: Least significant bit first (default).
#[register(address = Reg::Status, access_type = Ilps22qs, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct Status {
    #[bits(1, access = RO)]
    pub p_da: u8,
    #[bits(1, access = RO)]
    pub t_da: u8,
    #[bits(2, access = RO)]
    not_used_01: u8,
    #[bits(1, access = RO)]
    pub p_or: u8,
    #[bits(1, access = RO)]
    pub t_or: u8,
    #[bits(2, access = RO)]
    not_used_02: u8,
}

/// Pressure output register.
///
/// This register contains the raw pressure output value from the sensor. The value is a 24-bit signed integer
/// expressed in two's complement format, representing the measured pressure. It is composed of three registers:
/// PRESS_OUT_H, PRESS_OUT_L, and PRESS_OUT_XL. The pressure output value is provided as the difference between
/// the measured pressure and the offset stored in the RPDS registers.
///
/// # Fields
///
/// * `pout` - The raw pressure output value as a 32-bit signed integer (read-only).
#[register(address = Reg::PressOutXl, access_type = Ilps22qs, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u32, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u32, order = Lsb))]
pub struct PressOut {
    #[offset_before(8)]
    #[bits(32, access = RO)]
    pub pout: i32,
}

/// Temperature output register.
///
/// This register contains the raw temperature output value from the sensor. The value is a 16-bit signed integer
/// expressed in two's complement format, representing the measured temperature in units of 0.01 °C.
///
/// # Fields
///
/// * `tout` - The raw temperature output value as a 16-bit signed integer (read-only).
#[register(address = Reg::TempOutL, access_type = Ilps22qs, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u16, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u16, order = Lsb))]
pub struct TempOut {
    #[bits(16, access = RO)]
    pub tout: i16,
}

/// FIFO pressure data output register.
///
/// This register contains the raw pressure data stored in the FIFO buffer. The value is a 24-bit signed integer
/// expressed in two's complement format, representing the pressure measurement. It is composed of three registers:
/// FIFO_DATA_OUT_PRESS_H, FIFO_DATA_OUT_PRESS_L, and FIFO_DATA_OUT_PRESS_XL. The data is read-only and updated
/// as new samples are stored in the FIFO.
///
/// # Fields
///
/// * `fifo_p` - The raw FIFO pressure output value as a 32-bit signed integer (read-only).
#[register(address = Reg::FifoDataOutPressXl, access_type = Ilps22qs, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u32, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u32, order = Lsb))]
pub struct FifoDataOutPress {
    #[offset_before(8)]
    #[bits(32, access = RO)]
    pub fifo_p: i32,
}

/// Device identification register.
///
/// This register contains the "Who am I" identification value of the device. It is used to verify the presence
/// and correct communication with the sensor. The value is fixed and read-only.
///
/// # Fields
///
/// * `whoami` - The device identification value as an 8-bit unsigned integer (read-only).
#[register(address = Reg::WhoAmI, access_type = Ilps22qs, generics = 2)]
#[cfg_attr(feature = "bit_order_msb", bitfield(u8, order = Msb))]
#[cfg_attr(not(feature = "bit_order_msb"), bitfield(u8, order = Lsb))]
pub struct WhoAmI {
    #[bits(8, access = RO)]
    pub whoami: u8,
}

/// The `BusMode` struct represents the configuration settings for the communication interface and filter mode of the device.
///
/// This struct is used to specify and retrieve the desired bus operating mode, including the type of communication interface
/// and the filter settings.
pub struct BusMode {
    /// The communication interface type.
    pub interface: Interface,
    /// The filter settings for the communication interface.
    pub filter: Filter,
}

/// The `Stat` struct represents the comprehensive status information of the device.
///
/// This struct is used to store various status indicators that provide detailed insights into the device's
/// current operational state. It includes information about reset status, boot status, data readiness,
/// measurement completion, and reference operations.
pub struct Stat {
    /// Software reset status.
    pub sw_reset: u8,
    /// Boot status.
    pub boot: u8,
    /// Data readiness for pressure.
    pub drdy_pres: u8,
    /// Data readiness for temperature.
    pub drdy_temp: u8,
    /// Overrun status for pressure data.
    pub ovr_pres: u8,
    /// Overrun status for temperature data.
    pub ovr_temp: u8,
    /// Measurement completion status.
    pub end_meas: u8,
    /// Reference operation completion status.
    pub ref_done: u8,
}

/// The `PinConf` struct represents the electrical configuration settings for the device's configurable pins.
///
/// This struct is used to specify and retrieve the desired electrical settings for the device's pins,
/// such as enabling or disabling pull-up resistors.
pub struct PinConf {
    /// Indicates whether the pull-up resistor for the SDA pin is enabled.
    pub sda_pull_up: u8,
    /// Indicates whether the pull-up resistor for the CS pin is enabled.
    pub cs_pull_up: u8,
}

/// Represents the status of all interrupt sources for a device.
///
/// The `AllSources` struct provides detailed information about various interrupt conditions
/// that can occur in the device. It includes indicators for data readiness, pressure thresholds,
/// and FIFO conditions.
pub struct AllSources {
    /// Data readiness indicator for pressure measurements.
    pub drdy_pres: u8,
    /// Data readiness indicator for temperature measurements.
    pub drdy_temp: u8,
    /// Indicator for over-pressure condition.
    pub over_pres: u8,
    /// Indicator for under-pressure condition.
    pub under_pres: u8,
    /// Indicator for pressure threshold condition.
    pub thrsld_pres: u8,
    /// Indicator for FIFO full condition.
    pub fifo_full: u8,
    /// Indicator for FIFO overrun condition.
    pub fifo_ovr: u8,
    /// Indicator for FIFO threshold condition.
    pub fifo_th: u8,
}

/// Represents the sensor conversion parameters.
///
/// The `Md` struct encapsulates various settings related to sensor conversion, including output data rate (ODR),
/// averaging, low-pass filter settings, full-scale mode, and interleaved mode configuration. These parameters
/// are crucial for configuring the sensor's data processing and acquisition behavior.
#[derive(Default)]
pub struct Md {
    /// Specifies whether interleaved mode is enabled.
    pub interleaved_mode: u8,
    /// Represents the full-scale mode setting, which determines the pressure range.
    pub fs: Fs,
    /// Specifies the output data rate setting, which affects the frequency of data acquisition.
    pub odr: Odr,
    /// Represents the averaging setting, which determines the number of samples used for averaging.
    pub avg: Avg,
    /// Specifies the low-pass filter setting, which affects the filtering of sensor data.
    pub lpf: Lpf,
}

/// Represents AH/QVAR data retrieved from the sensor.
///
/// It includes both raw and processed values, allowing for comprehensive analysis and application-specific processing.
pub struct AhQvarData {
    /// The converted AH/QVAR value in millivolts (mV)
    pub mv: f32,
    /// The least significant byte (LSB) of the AH/QVAR data
    pub lsb: i32,
    /// The raw AH/QVAR data value
    pub raw: i32,
}

/// Represents the FIFO operation mode settings for the device.
///
/// The `FifoMd` struct encapsulates the configuration parameters for the FIFO (First-In, First-Out)
/// operation mode.
pub struct FifoMd {
    /// Specifies the FIFO operation mode
    pub operation: Operation,
    /// Defines the watermark level for the FIFO buffer
    pub watermark: u8,
}

/// Represents data retrieved from the FIFO buffer.
///
/// The `FifoData` struct encapsulates the processed data from the FIFO buffer, including both raw and
/// converted values. It supports the representation of pressure and AH/QVAR data, depending on the
/// sensor configuration.
#[derive(Default, Clone, Copy)]
pub struct FifoData {
    /// The converted pressure value in hectopascals (hPa)
    pub hpa: f32,
    /// The least significant byte (LSB) of the AH/QVAR data
    pub lsb: i32,
    /// The raw data value
    pub raw: i32,
}

/// Represents the configuration parameters for interrupt thresholds.
///
/// The `IntThMd` struct encapsulates the settings for the device's wake-up and wake-up-to-sleep
/// threshold events, which are based on pressure levels.
pub struct IntThMd {
    /// Specifies the pressure threshold value that triggers interrupt events.
    pub threshold: u16,
    /// Indicates whether an over-threshold event is enabled
    pub over_th: u8,
    /// Indicates whether an under-threshold event is enabled (0 or 1).
    pub under_th: u8,
}

/// Represents the reference mode settings for wake-up and wake-up-to-sleep functionality.
///
/// The `RefMd` struct encapsulates the configuration parameters for managing reference pressure levels,
/// which are used to trigger wake-up and sleep events.
pub struct RefMd {
    /// Specifies how reference pressure levels are applied.
    pub apply_ref: ApplyRef,
    /// Indicates whether the device should obtain reference pressure levels, influencing how reference configurations are managed
    pub get_ref: u8,
}

/// Represents pressure data retrieved from the sensor.
///
/// The `Pressure` struct encapsulates both raw and processed pressure data, providing a meaningful
/// representation in hectopascals (hPa) for applications that require pressure measurements.
#[derive(Clone, Copy, Default)]
pub struct Pressure {
    /// The converted pressure value in hectopascals (hPa).
    pub hpa: f32,
    /// The raw pressure data value.
    pub raw: i32,
}

/// Represents temperature data retrieved from the sensor.
///
/// The `Heat` struct encapsulates both raw and processed temperature data, providing a meaningful
/// representation in degrees Celsius (°C) for applications that require temperature measurements.
#[derive(Clone, Copy, Default)]
pub struct Heat {
    /// The converted temperature value in degrees Celsius (°C).
    pub deg_c: f32,
    /// The raw temperature data value.
    pub raw: i16,
}

/// Represents AH/QVAR data retrieved from the sensor.
///
/// The `AhQvar` struct encapsulates the least significant byte (LSB) of the AH/QVAR data, used for
/// detailed data analysis in advanced sensing applications.
#[derive(Clone, Copy, Default)]
pub struct AhQvar {
    /// The least significant byte (LSB) of the AH/QVAR data.
    pub lsb: i32,
}

/// Represents the complete set of sensor data, including pressure, temperature, and AH/QVAR measurements.
///
/// The `Data` struct aggregates the processed sensor data, providing a comprehensive view of the
/// sensor's output for applications that require multiple data types.
#[derive(Clone, Copy, Default)]
pub struct Data {
    /// Contains the processed pressure data, including both raw and converted values.
    pub pressure: Pressure,
    /// Contains the processed temperature data, including both raw and converted values.
    pub heat: Heat,
    /// Contains the processed AH/QVAR data, focusing on the least significant byte (LSB).
    pub ah_qvar: AhQvar,
}

/// Represents the communication interface mode for the device.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum Interface {
    /// The interface mode is selected by hardware configuration.
    #[default]
    SelByHw = 0x00,
    /// The device operates in 3-wire SPI mode.
    Spi3w = 0x03,
}

/// Represents the filter mode for the device's communication interface.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum Filter {
    /// The filter mode is automatically managed by the device.
    #[default]
    Auto = 0x00,
    /// The filter is always active, providing continuous signal filtering.
    AlwaysOn = 0x01,
}

/// Represents the initialization settings for the device.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum Init {
    /// Prepares the device for normal operation, ensuring it is ready to perform measurements.
    DrvRdy = 0x00,
    /// Initiates a boot procedure, typically used to restart the device and reload configuration settings.
    Boot = 0x01,
    /// Performs a software reset of the device, restoring it to its default state.
    Reset = 0x02,
}

/// Represents the full-scale mode settings for the sensor.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum Fs {
    /// Full-scale mode set to 1260 hectopascals (hPa), suitable for standard pressure measurements.
    #[default]
    _1260hpa = 0x00,
    /// Full-scale mode set to 4060 hectopascals (hPa), suitable for high-pressure measurements.
    _4060hpa = 0x01,
}

/// Represents the output data rate (ODR) settings for the sensor.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum Odr {
    /// Performs a single measurement on demand, suitable for applications requiring precise, on-demand data.
    #[default]
    OneShot = 0x00,
    /// Sets the data acquisition rate to 1 Hz.
    _1hz = 0x01,
    /// Sets the data acquisition rate to 4 Hz.
    _4hz = 0x02,
    /// Sets the data acquisition rate to 10 Hz.
    _10hz = 0x03,
    /// Sets the data acquisition rate to 25 Hz.
    _25hz = 0x04,
    /// Sets the data acquisition rate to 50 Hz.
    _50hz = 0x05,
    /// Sets the data acquisition rate to 75 Hz.
    _75hz = 0x06,
    /// Sets the data acquisition rate to 100 Hz.
    _100hz = 0x07,
    /// Sets the data acquisition rate to 200 Hz.
    _200hz = 0x08,
}

/// Represents the averaging settings for the sensor.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum Avg {
    /// Averages 4 samples.
    #[default]
    _4 = 0,
    /// Averages 8 samples.
    _8 = 1,
    /// Averages 16 samples.
    _16 = 2,
    /// Averages 32 samples.
    _32 = 3,
    /// Averages 64 samples.
    _64 = 4,
    /// Averages 128 samples.
    _128 = 5,
    /// Averages 256 samples.
    _256 = 6,
    /// Averages 512 samples.
    _512 = 7,
}

/// Represents the low-pass filter settings for the sensor.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum Lpf {
    /// Disables the low-pass filter, allowing all frequencies to pass through.
    #[default]
    Disable = 0,
    /// Applies a low-pass filter with a cutoff frequency at one-fourth of the output data rate (ODR).
    OdrDiv4 = 1,
    /// Applies a low-pass filter with a cutoff frequency at one-ninth of the output data rate (ODR).
    OdrDiv9 = 3,
}

/// Represents the FIFO operation modes for the device.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum Operation {
    /// The FIFO buffer is bypassed.
    #[default]
    Bypass = 0,
    /// Data is stored in the FIFO buffer.
    Fifo = 1,
    /// Data is streamed directly.
    Stream = 2,
    /// Data is streamed and then stored in the FIFO buffer.
    StreamToFifo = 7,
    /// The device transitions from bypass mode to stream mode.
    BypassToStream = 6,
    /// The device transitions from bypass mode to FIFO mode.
    BypassToFifo = 5,
}

/// Represents the application of reference pressure levels for wake-up and wake-up-to-sleep functionality.
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Default, TryFrom)]
#[try_from(repr)]
pub enum ApplyRef {
    /// Applies reference pressure levels to both output and interrupt signals.
    OutAndInterrupt = 0,
    /// Applies reference pressure levels only to interrupt signals.
    OnlyInterrupt = 1,
    /// Resets reference configurations, clearing any previously set reference levels.
    #[default]
    RstRefs = 2,
}
