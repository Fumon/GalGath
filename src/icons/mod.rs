use std::include_bytes;


pub const FAVICON: &'static [u8; include_bytes!("favicon.ico").len()] = include_bytes!("favicon.ico");
pub const ICON_48: &'static [u8; include_bytes!("android/mipmap-mdpi/ic_launcher.png").len()] = include_bytes!("android/mipmap-mdpi/ic_launcher.png");
pub const ICON_72: &'static [u8; include_bytes!("android/mipmap-hdpi/ic_launcher.png").len()] = include_bytes!("android/mipmap-hdpi/ic_launcher.png");
pub const ICON_96: &'static [u8; include_bytes!("android/mipmap-xhdpi/ic_launcher.png").len()] = include_bytes!("android/mipmap-xhdpi/ic_launcher.png");
pub const ICON_144: &'static [u8; include_bytes!("android/mipmap-xxhdpi/ic_launcher.png").len()] = include_bytes!("android/mipmap-xxhdpi/ic_launcher.png");
pub const ICON_192: &'static [u8; include_bytes!("android/mipmap-xxxhdpi/ic_launcher.png").len()] = include_bytes!("android/mipmap-xxxhdpi/ic_launcher.png");


pub const fn get_icon(size: i32) -> Option<&'static [u8]> {
    match size {
        48 => Some(ICON_48.as_slice()),
        72 => Some(ICON_72.as_slice()),
        96 => Some(ICON_96.as_slice()),
        144 => Some(ICON_144.as_slice()),
        192 => Some(ICON_192.as_slice()),
        _ => None
    }
}