pub mod gnome;
mod kde;
mod lxde;
pub(crate) mod xfce;

use crate::{get_stdout, run, Error, Mode, Result};
use std::{env, process::Command};

/// Returns the wallpaper of the current desktop.
pub fn get() -> Result<String> {
    let desktop = env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();

    if gnome::is_compliant(&desktop) {
        return gnome::get();
    }

    match desktop.as_str() {
        "KDE" => kde::get(),
        "X-Cinnamon" => parse_dconf(
            "dconf",
            &["read", "/org/cinnamon/desktop/background/picture-uri"],
        ),
        "MATE" => parse_dconf(
            "dconf",
            &["read", "/org/mate/desktop/background/picture-filename"],
        ),
        "XFCE" => xfce::get(),
        "LXDE" => lxde::get(),
        "Deepin" => parse_dconf(
            "dconf",
            &[
                "read",
                "/com/deepin/wrap/gnome/desktop/background/picture-uri",
            ],
        ),
        _ => Err(Error::UnsupportedDesktop),
    }
}

/// Sets the wallpaper for the current desktop from a file path.
pub fn set_from_path(path: &str) -> Result<()> {
    let desktop = env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();

    if gnome::is_compliant(&desktop) {
        return gnome::set(path);
    }

    match desktop.as_str() {
        "KDE" => kde::set(path),
        "X-Cinnamon" => run(
            "dconf",
            &[
                "write",
                "/org/cinnamon/desktop/background/picture-uri",
                &enquote::enquote('"', &format!("file://{}", path)),
            ],
        ),
        "MATE" => run(
            "dconf",
            &[
                "write",
                "/org/mate/desktop/background/picture-filename",
                &enquote::enquote('"', &path),
            ],
        ),
        "XFCE" => xfce::set(path),
        "LXDE" => lxde::set(path),
        "Deepin" => run(
            "dconf",
            &[
                "write",
                "/com/deepin/wrap/gnome/desktop/background/picture-uri",
                &enquote::enquote('"', &format!("file://{}", path)),
            ],
        ),
        _ => {
            if let Ok(mut child) = Command::new("swaybg").args(&["-i", path]).spawn() {
                child.stdout = None;
                child.stderr = None;
                return Ok(());
            }

            run("feh", &["--bg-fill", path])
        }
    }
}

pub fn get_dark() -> Result<String> {
    let desktop = env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();

    if gnome::is_compliant(&desktop) {
        return gnome::get_dark();
    }

    Err(Error::UnsupportedDesktop)
}

pub fn set_dark_from_path(path: &str) -> Result<()> {
    let desktop = env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();

    if gnome::is_compliant(&desktop) {
        return gnome::set_dark(path);
    }

    Err(Error::UnsupportedDesktop)
}

/// Sets the wallpaper style.
pub fn set_mode(mode: Mode) -> Result<()> {
    let desktop = env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();

    if gnome::is_compliant(&desktop) {
        return gnome::set_mode(mode);
    }

    match desktop.as_str() {
        "KDE" => kde::set_mode(mode),
        "X-Cinnamon" => run(
            "dconf",
            &[
                "write",
                "/org/cinnamon/desktop/background/picture-options",
                &mode.get_gnome_string(),
            ],
        ),
        "MATE" => run(
            "dconf",
            &[
                "write",
                "/org/mate/desktop/background/picture-options",
                &mode.get_gnome_string(),
            ],
        ),
        "XFCE" => xfce::set_mode(mode),
        "LXDE" => lxde::set_mode(mode),
        "Deepin" => run(
            "dconf",
            &[
                "write",
                "/com/deepin/wrap/gnome/desktop/background/picture-options",
                &mode.get_gnome_string(),
            ],
        ),
        _ => Err(Error::UnsupportedDesktop),
    }
}

pub fn get_mode() -> Result<Mode> {
    Err(Error::UnsupportedDesktop)
}

fn parse_dconf(command: &str, args: &[&str]) -> Result<String> {
    let mut stdout = enquote::unquote(&get_stdout(command, args)?)?;
    // removes file protocol
    if stdout.starts_with("file://") {
        stdout = stdout[7..].into();
    }
    Ok(stdout)
}
