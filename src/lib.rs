use i2cdev::core::{I2CMessage, I2CTransfer};
use i2cdev::linux::{LinuxI2CBus, LinuxI2CMessage};
use std::error::Error;

const ADDR: u16 = 0x5B;
const CCS811_HW_ID: u8 = 0x20;
const CCS811_APP_START: u8 = 0xF4;
const CCS811_ALG_RESULT_DATA: u8 = 0x02;

pub fn init() -> Result<LinuxI2CBus, Box<dyn Error>> {
    let path = "/dev/i2c-1";

    let mut bus = i2cdev::linux::LinuxI2CBus::new(path)?;

    configure_ccs811(&mut bus)?;

    Ok(bus)
}

pub fn read_co2(bus: &mut LinuxI2CBus) -> Result<u32, Box<dyn Error>> {
    let mut data = [0u8; 4];
    let mut msgs = [
        LinuxI2CMessage::write(&[CCS811_ALG_RESULT_DATA]).with_address(ADDR),
        LinuxI2CMessage::read(&mut data).with_address(ADDR),
    ];

    bus.transfer(&mut msgs)?;

    let [co2_msb, co2_lsb, _tvoc_msb, _tvoc_lsb] = data;

    // let co2 = co2_msb << 8 | co2_lsb;
    let co2 = (co2_msb as u32) << 0b1000u8 | (co2_lsb as u32);
    // tvoc = tvoc_msb <<< 8 ||| tvoc_lsb

    Ok(co2)
}

fn configure_ccs811(bus: &mut LinuxI2CBus) -> Result<(), Box<dyn Error>> {
    let mut data = [0; 1];
    let mut msgs = [
        LinuxI2CMessage::write(&[CCS811_HW_ID]).with_address(ADDR),
        LinuxI2CMessage::read(&mut data).with_address(ADDR),
    ];

    bus.transfer(&mut msgs)?;

    let hardware_id = data[0];

    if hardware_id != 0x81 {
        return Err("err".to_owned().into());
    }

    start_app(bus)?;

    std::thread::sleep(std::time::Duration::from_millis(100));

    Ok(())
}

fn start_app(bus: &mut LinuxI2CBus) -> Result<(), Box<dyn Error>> {
    let mut msgs = [LinuxI2CMessage::write(&[CCS811_APP_START]).with_address(ADDR)];

    bus.transfer(&mut msgs)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
