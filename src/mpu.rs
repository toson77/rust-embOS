use core::ptr::{read_volatile, write_volatile};
use cortex_m_semihosting::hprintln;

const MPU_TYPE_ADDR: usize = 0xE000_ED90;
const MPU_CTRL_ADDR: usize = 0xE000_ED94;
const MPU_RNR_ADDR: usize = 0xE000_ED98;
const MPU_RASR_ADDR: usize = 0xE000_EDA0;
const MPU_RBAR_ADDR: usize = 0xE000_ED9C;

pub fn stack_protect_thread1() {
    unsafe {
        //disable MPU
        write_volatile(MPU_CTRL_ADDR as *mut u32, 0x0000_0000);

        //regions2(protect all stack unprivileged Read only )
        write_volatile(MPU_RNR_ADDR as *mut u32, 0x0000_0001);
        write_volatile(MPU_RBAR_ADDR as *mut u32, 0x2000_0011);

        write_volatile(
            MPU_RASR_ADDR as *mut u32,
            0b000_0_0_010_00_001_0_0_0_00000000_00_11100_1,
        );
        //regions8(allow access stack)
        write_volatile(MPU_RNR_ADDR as *mut u32, 0x0000_0007);
        write_volatile(MPU_RBAR_ADDR as *mut u32, 0x2000_0017);
        write_volatile(
            MPU_RASR_ADDR as *mut u32,
            0b000_0_0_011_00_001_0_0_0_00000000_00_01010_1,
        );

        //enable MPU
        write_volatile(MPU_CTRL_ADDR as *mut u32, 0x0000_0001);
    }
}

pub fn stack_protect_thread2() {
    unsafe {
        //disable MPU
        write_volatile(MPU_CTRL_ADDR as *mut u32, 0x0000_0000);

        //regions2(protect all stack unprivileged Read only )
        write_volatile(MPU_RNR_ADDR as *mut u32, 0x0000_0001);
        write_volatile(MPU_RBAR_ADDR as *mut u32, 0x2000_0011);
        write_volatile(
            MPU_RASR_ADDR as *mut u32,
            0b000_0_0_010_00_001_0_0_0_00000000_00_11100_1,
        );

        //regions8(allow access stack)
        write_volatile(MPU_RNR_ADDR as *mut u32, 0x0000_0007);
        write_volatile(MPU_RBAR_ADDR as *mut u32, 0x2000_0817);
        write_volatile(
            MPU_RASR_ADDR as *mut u32,
            0b000_0_0_011_00_001_0_0_0_00000000_00_01010_1,
        );

        //enable MPU
        write_volatile(MPU_CTRL_ADDR as *mut u32, 0x0000_0001);
    }
}

pub fn stack_protect_thread3() {
    unsafe {
        //disable MPU
        write_volatile(MPU_CTRL_ADDR as *mut u32, 0x0000_0000);

        //regions2(protect all stack unprivileged Read only )
        write_volatile(MPU_RNR_ADDR as *mut u32, 0x0000_0001);
        write_volatile(MPU_RBAR_ADDR as *mut u32, 0x2000_0011);
        write_volatile(
            MPU_RASR_ADDR as *mut u32,
            0b000_0_0_010_00_001_0_0_0_00000000_00_11100_1,
        );

        //regions8(allow access stack)
        write_volatile(MPU_RNR_ADDR as *mut u32, 0x0000_0007);
        write_volatile(MPU_RBAR_ADDR as *mut u32, 0x2000_1017);
        write_volatile(
            MPU_RASR_ADDR as *mut u32,
            0b000_0_0_011_00_001_0_0_0_00000000_00_01010_1,
        );

        //enable MPU
        write_volatile(MPU_CTRL_ADDR as *mut u32, 0x0000_0001);
    }
}

pub fn init() {
    hprintln!("enabling MPU");
    let mpu_type_value = unsafe { read_volatile(MPU_TYPE_ADDR as *const u32) };
    if mpu_type_value != 0 {
        hprintln!("MPU exists");
    } else {
        hprintln!("MPU doesn't exist");
    }
    unsafe {
        //disable MPU
        write_volatile(MPU_CTRL_ADDR as *mut u32, 0x0000_0000);

        //setting on regions(8 regions)
        //regions1
        write_volatile(MPU_RNR_ADDR as *mut u32, 0x0000_0000);
        write_volatile(MPU_RBAR_ADDR as *mut u32, 0x0000_0010);
        //XN_AP_TEX_S_C_B_SRD_SIZE_EN
        write_volatile(
            MPU_RASR_ADDR as *mut u32,
            0b000_0_0_011_00_001_0_0_0_00000000_00_11100_1,
        );
        //regions2
        write_volatile(MPU_RNR_ADDR as *mut u32, 0x0000_0001);
        write_volatile(MPU_RBAR_ADDR as *mut u32, 0x2000_0011);
        write_volatile(
            MPU_RASR_ADDR as *mut u32,
            0b000_0_0_011_00_001_0_0_0_00000000_00_11100_1,
        );

        //regions3(gpio privilege)
        write_volatile(MPU_RNR_ADDR as *mut u32, 0x0000_0002);
        write_volatile(MPU_RBAR_ADDR as *mut u32, 0x4000_0012);
        write_volatile(
            MPU_RASR_ADDR as *mut u32,
            0b000_0_0_001_00_001_0_0_0_00000000_00_11100_1,
        );

        //regions4
        write_volatile(MPU_RNR_ADDR as *mut u32, 0x0000_0003);
        write_volatile(MPU_RBAR_ADDR as *mut u32, 0x6000_0013);
        write_volatile(
            MPU_RASR_ADDR as *mut u32,
            0b000_0_0_011_00_001_0_0_0_00000000_00_11100_1,
        );

        //regions5
        write_volatile(MPU_RNR_ADDR as *mut u32, 0x0000_0004);
        write_volatile(MPU_RBAR_ADDR as *mut u32, 0x8000_0014);
        write_volatile(
            MPU_RASR_ADDR as *mut u32,
            0b000_0_0_011_00_001_0_0_0_00000000_00_11100_1,
        );

        //regions6
        write_volatile(MPU_RNR_ADDR as *mut u32, 0x0000_0005);
        write_volatile(MPU_RBAR_ADDR as *mut u32, 0xA000_0015);
        write_volatile(
            MPU_RASR_ADDR as *mut u32,
            0b000_0_0_011_00_001_0_0_0_00000000_00_11100_1,
        );

        //regions7
        write_volatile(MPU_RNR_ADDR as *mut u32, 0x0000_0006);
        write_volatile(MPU_RBAR_ADDR as *mut u32, 0xD000_0016);
        write_volatile(
            MPU_RASR_ADDR as *mut u32,
            0b000_0_0_011_00_001_0_0_0_00000000_00_11100_1,
        );

        //regions8
        write_volatile(MPU_RNR_ADDR as *mut u32, 0x0000_0007);
        write_volatile(MPU_RBAR_ADDR as *mut u32, 0xE000_0017);
        write_volatile(
            MPU_RASR_ADDR as *mut u32,
            0b000_0_0_011_00_001_0_0_0_00000000_00_11100_1,
        );

        //enable MPU
        write_volatile(MPU_CTRL_ADDR as *mut u32, 0x0000_0001);
        stack_protect_thread1();
    }
}
