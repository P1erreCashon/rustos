use crate::sbi::shutdown;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        if let message = info.message() {
            println!(
                "Panicked at {}:{} {}",
                location.file(),
                location.line(),
                message
            );
        } else {
            println!(
                "Panicked at {}:{} <no message>",
                location.file(),
                location.line()
            );
        }
    } else {
        if let message = info.message() {
            println!("Panicked: {}", message);
        } else {
            println!("Panicked: <no message>");
        }
    }
    shutdown(true)
}