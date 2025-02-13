use crate::{app, sys};
use notblib::{get, stat};

pub fn run() {
    stat::process("Preparing for work...");

    // Creating default config in not exists
    if !app::cfg::exists() {
        if {
            app::cfg::Config {
                version: sys::cfg::VERSION.into(),
                net: app::cfg::NetConfig {
                    host: sys::cfg::DEFAULT_HOST.into(),
                    port: sys::cfg::DEFAULT_PORT.into(),
                    proto: sys::cfg::DEFAULT_PROTOCOL.into(),
                },
            }
            .set()
        }
        .is_err()
        {
            stat::fatal("Error setting default config");
        }
    }
}
