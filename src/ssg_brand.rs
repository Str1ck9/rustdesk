use base::config::keys;
use hbb_common::config;

// No spaces: the Windows install/uninstall scripts pass the app name unquoted
// to `sc` and `taskkill`.
const APP_NAME: &str = "SSGRemoteSupport";
const SERVER: &str = "help.stricklandservicesgroup.com";
const API_SERVER: &str = "https://help.stricklandservicesgroup.com";
// Read by the Flutter home page (desktop_home_page.dart).
const HIDE_INSTALL_DAEMON_TIP: &str = "ssg-hide-install-daemon-tip";

// The hbbs public key (id_ed25519.pub) comes from the build environment so it
// can be rotated without a code change. Release builds refuse to compile
// without it, so a client that cannot verify the server never ships.
#[cfg(not(debug_assertions))]
const KEY: &str = env!("SSG_RS_PUB_KEY");
#[cfg(debug_assertions)]
const KEY: &str = match option_env!("SSG_RS_PUB_KEY") {
    Some(key) => key,
    None => "",
};

/// Applies the same overrides a signed `custom.txt` would, without needing a
/// RustDesk Server Pro signature.
pub fn apply() {
    *config::APP_NAME.write().unwrap() = APP_NAME.to_owned();
    let mut settings = config::OVERWRITE_SETTINGS.write().unwrap();
    settings.insert(
        keys::OPTION_CUSTOM_RENDEZVOUS_SERVER.to_owned(),
        SERVER.to_owned(),
    );
    settings.insert(keys::OPTION_RELAY_SERVER.to_owned(), SERVER.to_owned());
    settings.insert(keys::OPTION_API_SERVER.to_owned(), API_SERVER.to_owned());
    if !KEY.is_empty() {
        settings.insert(keys::OPTION_KEY.to_owned(), KEY.to_owned());
    }
    drop(settings);
    // SSG never ships the macOS start-on-boot daemon, so its install card is
    // only noise for technicians and customers alike.
    config::BUILTIN_SETTINGS
        .write()
        .unwrap()
        .insert(HIDE_INSTALL_DAEMON_TIP.to_owned(), "Y".to_owned());
}
