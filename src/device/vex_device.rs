use serialport::SerialPortBuilder;
// use super::packets::TXPayload;
use std::time::SystemTime;

// This is all ported from pros

pub trait VexDevice: super::GenericDevice + super::SystemDevice {
    fn set_default_timeout(&self, timeout: f32);
    fn get_default_timeout(&self) -> f32;


    fn vex_init(&self, port: &SerialPortBuilder, timeout: f32) {
        self.generic_init(port);
        self.set_default_timeout(timeout);
    }

    fn form_simple_packet(&self, msg: u8) -> [u8; 5] {
        [0xc9, 0x36, 0xb8, 0x47, msg]
    }

    fn rx_packet(&self, timeout_p: Option<f32>) -> Result<(u8, Vec<u8>, Vec<u8>), ()> {
        let now = SystemTime::now();

        let response_header: [u8; 2] = [0xAA, 0x55];
        let mut response_header_stack = Vec::from(response_header);

        let mut rx: Vec<u8> = Vec::new();

        let timeout;
        if timeout_p.is_none() {
            timeout = self.get_default_timeout();
        } else {
            timeout = timeout_p.unwrap();
        }
        
        let mut port = self.get_port().open().expect("Failed to open port");

        while (rx.len() > 0 || now.elapsed().expect("failed to get time").as_micros() < timeout as u128) && response_header_stack.len() > 0 {
            let mut buf: [u8; 1] = [0; 1];
            port.read(&mut buf).expect("failed to read");

            if buf.len() == 0 {
                continue;
            }

            let byte = buf[0];
            if byte == response_header_stack[0] {
                response_header_stack.remove(0);
                rx.append(&mut Vec::from(buf));
            } else {
                response_header_stack = Vec::from(response_header);
                rx = Vec::new();
            }
        }
        if rx != Vec::from(response_header) {
            return Err(());
        }

        let mut buf: Vec<u8> = Vec::new();
        port.read(&mut buf).expect("Failed to read port");
        rx.append(&mut buf);

        let command = *rx.last().expect("Failed to get last element");

        let mut buf: Vec<u8> = Vec::new();
        port.read(&mut buf).expect("Failed to read port");
        rx.append(&mut buf);

        let mut payload_len = *rx.last().expect("Failed to get last element") as u32;

        if command == (0x56 as u8) && (payload_len & 0x80) == 0x80 {
            let mut buf: Vec<u8> = Vec::new();
            port.read(&mut buf).expect("Failed to read port");
            rx.append(&mut buf);

            payload_len = ((payload_len & 0x7f) << 8) + (*rx.last().expect("Failed to get last element") as u32);
        }

        let mut payload: Vec<u8> = vec![0u8; payload_len as usize];
        port.read(payload.as_mut_slice()).expect("Failed to read port");


        Ok((
            command,
            payload,
            rx
        ))
    }

    fn tx_packet(&self, command: u8, tx_data: Option<Vec<u8>>) -> Vec<u8> {
        let mut tx = self.form_simple_packet(command).to_vec();
        if !tx_data.is_none() {
            let tx_data = tx_data.expect("tx_data was none when it wasn't none");
            tx = [tx_data, tx].concat();
        }
        let mut port = self.get_port().open().expect("Failed to open port");

        port.read_to_end(&mut Vec::new()).expect("Failed to read rest");
        port.write(tx.as_slice()).expect("Failed to write");
        port.flush().expect("Failed to flush port");

        tx
    }

    fn txrx_packet(&self, command: u8, tx_data: Option<Vec<u8>>, timeout: Option<f32>) -> (u8, Vec<u8>) {
        self.tx_packet(command, tx_data);
        let (rx_command, rx_payload, _) = 
            self.rx_packet(timeout)
            .expect("Failed to read response");

        (rx_command, rx_payload)
    }

    fn txrx_simple_packet(&self, command: u8, rx_len: u8, timeout: Option<f32>) -> Result<Vec<u8>, ()> {
        let (msg_command, msg_payload) = self.txrx_packet(command, None, timeout);
        if msg_command != command {
            return Err(());
        }
        if msg_payload.len() != rx_len.into() {
            return Err(());
        }

        Ok(msg_payload)
    }

    fn txrx_simple_struct<T>(&self, command: u8, unpack_fmt: for<'r> fn(&'r &[u8]) -> T, length: u8, timeout: Option<f32>) -> T {
        let rx = self.txrx_simple_packet(command, length, timeout).expect("Failed to send packet");
        unpack_fmt(&rx.as_slice())
    }

    fn query_system(&self) -> Vec<u8> {
        self.txrx_simple_packet(0x21, 0x0A, None).expect("Failed to send packet")
    }

}