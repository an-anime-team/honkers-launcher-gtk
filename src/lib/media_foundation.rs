use std::process::{Command, Stdio, Output};

use anime_game_core::installer::installer::Installer;

use super::config;

/// Install media foundation fix for wine prefix
/// 
/// This fix required to run the game
/// 
/// Link: https://github.com/z0z0z/mf-install
pub fn install_media_foundation<T: ToString>(runners_folder: T, runner: super::wine::Version, prefix: T) -> anyhow::Result<Output> {
    let config = config::get()?;

    // Link to specific commit for security reasons
    let mut installer = Installer::new("https://github.com/z0z0z/mf-install/archive/f8d24e9b600bad038911e8618721c8bfb83872e9.zip")?;

    if let Some(temp) = &config.launcher.temp {
        installer = installer.set_temp_folder(temp);
    }

    // Temp media foundation folder
    let foundation_folder = format!("{}/.media-foundation", config.launcher.temp.unwrap_or(String::from("/tmp")));

    // It's 7.6 MB so I don't think we need any progress bar here
    installer.install(&foundation_folder, |_| {});

    let install_script = format!("{foundation_folder}/mf-install-f8d24e9b600bad038911e8618721c8bfb83872e9/mf-install.sh");

    // Change wine and wine64 binaries to selected runner
    std::fs::write(
        &install_script,
        std::fs::read_to_string(&install_script)?
            .replace("wine64 ", &format!("'{}/{}/{}' ", runners_folder.to_string(), runner.name, runner.files.wine64))
            .replace("wine ", &format!("'{}/{}/{}' ", runners_folder.to_string(), runner.name, runner.files.wine))
    )?;

    // Apply media foundation patch
    let output = Command::new(install_script)
        .env("WINEPREFIX", prefix.to_string())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;

    // Remove temp folder
    std::fs::remove_dir_all(foundation_folder)?;

    Ok(output)
}
