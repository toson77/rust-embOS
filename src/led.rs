use core::ptr::{read_volatile, write_volatile};

const GPIO_A_ADDR: usize = 0x4002_0000;
const GPIO_MODER_A_DEFAULT: usize = 0xA800_0000;
const GPIO_MODER_OFFSET: usize = 0x00;
const GPIO_ODR_OFFSET: usize = 0x14;
const LED_PIN: usize = 5;
const RCC_ADDR: usize = 0x4002_3800;
const RCC_AHB1ENR_OFFSET: usize = 0x30;

pub fn init() {
    unsafe {
        write_volatile((RCC_ADDR + RCC_AHB1ENR_OFFSET) as *mut u32, 1);
        let writingValue: u32 = (GPIO_MODER_A_DEFAULT | (0x01 << (LED_PIN * 2))) as u32;
        write_volatile((GPIO_A_ADDR + GPIO_MODER_OFFSET) as *mut u32, writingValue);
    }
}

pub fn turn_on() {
    unsafe {
        let reg_value: u32 = read_volatile((GPIO_A_ADDR + GPIO_ODR_OFFSET) as *mut u32);
        let writing_value: u32 = reg_value | (0x1 << (LED_PIN));
        write_volatile((GPIO_A_ADDR + GPIO_ODR_OFFSET) as *mut u32, writing_value);
    }
}
