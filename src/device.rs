pub mod packets;

pub mod vex_device;
pub mod v5_device;

pub trait GenericDevice {
    fn get_port(&self) -> serialport::SerialPortBuilder;
    fn set_port(&self, port: &serialport::SerialPortBuilder);

    fn generic_init(&self, port: &serialport::SerialPortBuilder){
        self.set_port(port);
    }
}

pub trait SystemDevice {
    fn upload_project(&self);
    fn write_program(&self, file: [u8], quirk: i32);
}