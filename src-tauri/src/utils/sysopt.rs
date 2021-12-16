use serde::{Deserialize, Serialize};
use std::io;

#[derive(Debug, Deserialize, Serialize)]
pub struct SysProxyConfig {
  enable: bool,
  server: String,
  bypass: String,
}

#[cfg(target_os = "windows")]
mod win {
  use super::*;
  use winreg::enums::*;
  use winreg::RegKey;

  /// Get the windows system proxy config
  pub fn get_proxy_config() -> io::Result<SysProxyConfig> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let cur_var = hkcu.open_subkey_with_flags(
      "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Internet Settings",
      KEY_READ,
    )?;

    Ok(SysProxyConfig {
      enable: cur_var.get_value::<u32, _>("ProxyEnable")? == 1u32,
      server: cur_var.get_value("ProxyServer")?,
      bypass: cur_var.get_value("ProxyOverride")?,
    })
  }

  /// Set the windows system proxy config
  pub fn set_proxy_config(config: &SysProxyConfig) -> io::Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let cur_var = hkcu.open_subkey_with_flags(
      "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Internet Settings",
      KEY_SET_VALUE,
    )?;

    let enable: u32 = if config.enable { 1u32 } else { 0u32 };

    cur_var.set_value("ProxyEnable", &enable)?;
    cur_var.set_value("ProxyServer", &config.server)?;
    cur_var.set_value("ProxyOverride", &config.bypass)?;

    Ok(())
  }
}

#[cfg(target_os = "macos")]
mod macos {
  use super::*;

  pub fn get_proxy_config() -> io::Result<SysProxyConfig> {
    Ok(SysProxyConfig {
      enable: false,
      server: "server".into(),
      bypass: "bypass".into(),
    })
  }

  pub fn set_proxy_config(config: &SysProxyConfig) -> io::Result<()> {
    Ok(())
  }
}

#[cfg(target_os = "windows")]
pub use win::*;

#[cfg(target_os = "macos")]
pub use macos::*;