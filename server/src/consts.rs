// SPDX-License-Identifier: GPL-2.0-or-later

#[cfg(windows)]
static CONFIG_PATH: &str = "%appdata%\\Roaming\\sims-server\\config.toml";

#[cfg(unix)]
static CONFIG_PATH: &str = "$HOME/.config/sims-server/config.toml";
