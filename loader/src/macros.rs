macro_rules! print {
    ($($arg:tt)*) => {{
        use crate::uefi::WRITER;
        if let Some(ref mut writer) = *WRITER.writer.borrow_mut() {
            _ = write!(writer, "{}", format_args!($($arg)*));
        }
    }};
}

macro_rules! println {
    () => {
        print!("\n");
    };
    ($($arg:tt)*) => {
        print!("{}\n", format_args!($($arg)*));
    };
}
