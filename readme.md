# wallpaper [![crate](https://img.shields.io/crates/v/wallpaper.svg)](https://crates.io/crates/wallpaper) [![docs](https://docs.rs/wallpaper/badge.svg)](https://docs.rs/wallpaper)

This Rust library gets and sets the desktop wallpaper/background.

The supported desktops are:

- Windows
- macOS
- GNOME
- KDE
- Cinnamon
- Unity
- Budgie
- XFCE
- LXDE
- MATE
- Deepin
- Most Wayland compositors (set only, requires swaybg)
- i3 (set only, requires feh)

## Examples

```rust
use wallpaper;

fn main() {
    // Returns the wallpaper of the current desktop.
    println!("{:?}", wallpaper::get());
    // Sets the wallpaper for the current desktop from a file path.
    wallpaper::set_from_path("/usr/share/backgrounds/gnome/adwaita-day.png").unwrap();
    // Sets the wallpaper style.
    wallpaper::set_mode(wallpaper::Mode::Crop).unwrap();
    // Returns the wallpaper of the current desktop.
    println!("{:?}", wallpaper::get());
}
```


