mod bindings {
    windows::include_bindings!();
}

// bindings from build script
use bindings::Windows::Win32::{
    Foundation::POINT,
    Graphics::Gdi::{GetDC, GetPixel, ReleaseDC},
    UI::{KeyboardAndMouseInput::GetActiveWindow, WindowsAndMessaging::GetCursorPos},
};
// structs
use bindings::Windows::Win32::{Foundation::HWND, Graphics::Gdi::HDC};

use json;
use std::{env, num::ParseIntError};

fn main() -> windows::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut lppoint: POINT = POINT::default();
    match args.len() {
        1 => unsafe {
            GetCursorPos(&mut lppoint);
        },
        3 => {
            let x: Result<i32, ParseIntError> = args[1].parse();
            if x.is_err() {
                return Err(windows::Error::new(
                    windows::HRESULT(0x80070057),
                    "argument 1 must be an integer",
                ));
            }
            lppoint.x = x.unwrap();
            let y: Result<i32, ParseIntError> = args[2].parse();
            if y.is_err() {
                return Err(windows::Error::new(
                    windows::HRESULT(0x80070057),
                    "argument 2 must be an integer",
                ));
            }
            lppoint.y = y.unwrap();
        }
        _ => {
            return Err(windows::Error::new(
                windows::HRESULT(0x80070057),
                "expected 0 or 2 arguments",
            ));
        }
    }
    let pixel: u32;
    unsafe {
        let hwnd: HWND = GetActiveWindow();
        let hdc: HDC = GetDC(hwnd);
        pixel = GetPixel(hdc, lppoint.x, lppoint.y);
        ReleaseDC(hwnd, hdc);
    }
    let color: u32 =
        ((pixel & 0xFF) << 16) | (((pixel & 0xFF00) >> 8) << 8) | ((pixel & 0xFF0000) >> 16); // switch color channels from bgr to rbg
    let mut data = json::JsonValue::new_object();
    data["x"] = lppoint.x.into();
    data["y"] = lppoint.y.into();
    data["color"] = json::JsonValue::new_object();
    data["color"]["u32"] = color.into();
    data["color"]["hex"] = format!("{:0>6X}", color).into();
    println!("{}", data);
    Ok(())
}
