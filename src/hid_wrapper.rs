use core::mem;

use usbd_human_interface_device::page::{Consumer, Keyboard};

pub fn keyboard_from_u8(key_value: u8) -> Option<Keyboard> {
    if (0x00..=0xA4).contains(&key_value) || (0xE0..=0xE7).contains(&key_value) {
        Some(unsafe { mem::transmute(key_value) })
    } else {
        None
    }
}

pub fn consumer_from_u16(consumer_value: u16) -> Option<Consumer> {
    if (0x00..=0x06).contains(&consumer_value)
        || (0x20..=0x22).contains(&consumer_value)
        || (0x30..=0x36).contains(&consumer_value)
        || (0x40..=0x48).contains(&consumer_value)
        || (0x60..=0x66).contains(&consumer_value)
        || (0x80..=0x9E).contains(&consumer_value)
        || (0x90..=0x92).contains(&consumer_value)
        || (0xA0..=0xA4).contains(&consumer_value)
        || (0xB0..=0xCE).contains(&consumer_value)
        || (0xE0..=0xEA).contains(&consumer_value)
        || (0xF0..=0xF5).contains(&consumer_value)
        || (0x100..=0x10D).contains(&consumer_value)
        || (0x150..=0x155).contains(&consumer_value)
        || (0x160..=0x16A).contains(&consumer_value)
        || (0x170..=0x174).contains(&consumer_value)
        || (0x180..=0x1BA).contains(&consumer_value)
        || (0x1BC..=0x1C7).contains(&consumer_value)
        || (0x200..=0x29C).contains(&consumer_value)
    {
        Some(unsafe { mem::transmute(consumer_value) })
    } else {
        None
    }
}
