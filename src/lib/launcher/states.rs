use anime_game_core::prelude::*;
use anime_game_core::honkai::prelude::*;

use crate::lib::config;
use crate::lib::wine_prefix::WinePrefix;

#[derive(Debug, Clone)]
pub enum LauncherState {
    Launch,

    WineNotInstalled,
    PrefixNotExists,

    // Always contains `VersionDiff::Diff`
    GameUpdateAvailable(VersionDiff),

    /// Always contains `VersionDiff::Outdated`
    GameOutdated(VersionDiff),

    /// Always contains `VersionDiff::NotInstalled`
    GameNotInstalled(VersionDiff)
}

impl Default for LauncherState {
    fn default() -> Self {
        Self::Launch
    }
}

impl LauncherState {
    pub fn get<T: Fn(&str)>(status: T) -> anyhow::Result<Self> {
        let config = config::get()?;

        // Check wine existence
        if let None = config.try_get_wine_executable() {
            return Ok(Self::WineNotInstalled);
        }

        // Check prefix existence
        if !WinePrefix::exists_in(&config.game.wine.prefix) {
            return Ok(Self::PrefixNotExists);
        }

        // Check game installation status
        status("Updating game info...");

        let game = Game::new(&config.game.path);
        let diff = game.try_get_diff()?;

        Ok(match diff {
            // We don't check Predownload here because hon-kai
            // seems to not have game pre-downloadings at all
            VersionDiff::Latest(_) | VersionDiff::Predownload { .. } => Self::Launch,

            VersionDiff::Diff { .. } => Self::GameUpdateAvailable(diff),
            VersionDiff::Outdated { .. } => Self::GameOutdated(diff),
            VersionDiff::NotInstalled { .. } => Self::GameNotInstalled(diff)
        })
    }
}
