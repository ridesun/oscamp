//! Standard library macros

/// Prints to the standard output.
///
/// Equivalent to the [`println!`] macro except that a newline is not printed at
/// the end of the message.
///
/// [`println!`]: crate::println
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::io::__print_impl(format_args!($($arg)*));
    }
}

/// Prints to the standard output, with a newline.
#[macro_export]
macro_rules! println {
    () => { $crate::print!("\n") };
    ($($arg:tt)*) => {
        // $crate::io::__print_impl(format_args!("{}\n", format_args!($($arg)*)));
        $crate::println_color!("#EE82EE", "#00D1FF", $($arg)*);
    }
}

#[macro_export]
macro_rules! println_color {
        ($start_color:expr, $end_color:expr, $($arg:tt)*) => ({

        let text = $crate::format!($($arg)*);
        let chars: $crate::vec::Vec<char> = text.chars().collect();
        let len = chars.len();

        struct RGB {
            r: u8,
            g: u8,
            b: u8,
        }

        fn parse_color(color: &str) -> RGB {
            let color = color.trim_start_matches('#');
            RGB {
                r: u8::from_str_radix(&color[0..2], 16).unwrap_or(0),
                g: u8::from_str_radix(&color[2..4], 16).unwrap_or(0),
                b: u8::from_str_radix(&color[4..6], 16).unwrap_or(0),
            }
        }

        let start = parse_color($start_color);
        let end = parse_color($end_color);
        $crate::os::arceos::modules::axlog::debug!("println_color: {} {}->{}", text,$start_color,$end_color);

        for (i, c) in chars.iter().enumerate() {
            let progress = if len > 1 { i as f32 / (len - 1) as f32 } else { 0.0 };

            let r = (start.r as f32 + (end.r as f32 - start.r as f32) * progress) as u8;
            let g = (start.g as f32 + (end.g as f32 - start.g as f32) * progress) as u8;
            let b = (start.b as f32 + (end.b as f32 - start.b as f32) * progress) as u8;

            $crate::print!("\x1b[38;2;{};{};{}m{}", r, g, b, c);
        }

        $crate::print!("\x1b[0m");
        $crate::print!("\n");
    })
}
