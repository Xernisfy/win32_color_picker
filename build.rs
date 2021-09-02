fn main() {
    windows::build! {
        Windows::Win32::{
          Foundation::POINT,
          Graphics::Gdi::{GetDC, GetPixel, ReleaseDC},
          UI::{KeyboardAndMouseInput::GetActiveWindow, WindowsAndMessaging::GetCursorPos},
      }
    };
}
