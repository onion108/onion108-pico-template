pub struct SerialManager {
    device: UsbDevice<'static, hal::usb::UsbBus>,
    serial: SerialPort<'static, hal::usb::UsbBus>,
}

impl SerialManager {
    pub fn new(usb_bus: &'static UsbBusAllocator<hal::usb::UsbBus>) -> Self {
        let serial = usbd_serial::SerialPort::new(usb_bus);

        let device = UsbDeviceBuilder::new(usb_bus, UsbVidPid(0x2E8A, 0x000a))
            .manufacturer("Raspberry Pi")
            .product("Pico")
            .serial_number("TEST")
            .device_class(2)
            .device_protocol(1)
            .build();

        SerialManager { device, serial }
    }

    pub unsafe fn interrupt(&mut self) {
        if self.device.poll(&mut [&mut self.serial]) {}
    }
}

impl core::fmt::Write for SerialManager {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.serial.write(s.as_bytes()).unwrap();

        Ok(())
    }
}

