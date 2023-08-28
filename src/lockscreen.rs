use windows::{core::HSTRING, Storage::StorageFile, System::UserProfile::LockScreen};

pub fn set_image(image_path: &str) {
    let image_file = StorageFile::GetFileFromPathAsync(&HSTRING::from(image_path))
        .unwrap()
        .get()
        .unwrap();

    LockScreen::SetImageFileAsync(&image_file)
        .unwrap()
        .get()
        .unwrap();
}
