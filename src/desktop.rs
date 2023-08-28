use windows::{
    core::{HSTRING, PCWSTR},
    Win32::{
        System::Com::{CoCreateInstance, CoInitialize, CLSCTX_ALL},
        UI::Shell::{DesktopWallpaper, IDesktopWallpaper},
    },
};

pub unsafe fn set_image(image_path: &str) {
    CoInitialize(None).unwrap();

    let wallpaper: IDesktopWallpaper =
        CoCreateInstance(&DesktopWallpaper, None, CLSCTX_ALL).unwrap();

    wallpaper
        .SetWallpaper(PCWSTR::null(), &HSTRING::from(image_path))
        .unwrap();
}
