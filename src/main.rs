mod android;
mod elf;
mod emulator;
mod keystone;
mod linux;
mod memory;
mod pointer;
mod svc_memory;
mod tool;

use crate::android::jni_graphics::register_jni_graphics;
use crate::emulator::AndroidEmulator;
use log::{error, info};
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;
use unicorn_engine::unicorn_const::{Arch, Mode};

fn main() {
    if let Err(_e) = std::env::var("RUST_LOG") {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    let emu = AndroidEmulator::create_arm64(32267, 29427, "com.tencent.mobileqq:MSF", ());
    let dm = emu
        .load_library(".\\txlib\\9.0.81\\libfekit.so", true)
        .map_err(|e| error!("failed to load_library: {}", e))
        .unwrap();

    let dm = unsafe { &*dm.get() };
    let _ = emu.call_jni_onload(&dm);
    
    sleep(Duration::from_secs(1000));
}

#[test]
fn test_unicorn() {
    use unicorn_engine::unicorn_const::{Arch, Mode, Permission, SECOND_SCALE};
    use unicorn_engine::RegisterARM;

    let mut unicorn = unicorn_engine::Unicorn::new(Arch::ARM, Mode::LITTLE_ENDIAN)
        .expect("failed to initialize Unicorn instance");

    unicorn
        .mem_map(0x8000, 0x4000, Permission::ALL)
        .expect("failed to map code page");

    let u2 = unicorn.clone();
    let data = [1u8, 2, 3, 4];
    u2.mem_write(0x8000, &data).expect("failed to write memory");

    let mut i32_buf = vec![0u8; 4];
    unicorn
        .mem_read(0x8000, i32_buf.as_mut_slice())
        .expect("failed to read memory");
    println!("BUF = {:?}", hex::encode(i32_buf));
}
