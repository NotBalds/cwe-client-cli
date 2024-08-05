use crate::{base, modules};

pub fn run() {
    base::log("Client info:", 4);
    base::sleep(0.03);
    base::log(&format!("Time: {}", base::unix_time()), 4);
    base::sleep(0.03);
    base::log(&format!("Server: {}", modules::config::default_url()), 4);
    base::sleep(0.03);
    base::log(&format!("UUID: {}", base::uuid::get()), 4);
    base::sleep(0.03);
    base::log(
        &format!(
            "PubKey: \n{}",
            base::filesystem::cat(&base::filesystem::new_path("base-keys/my-key.pub"))
        ),
        4,
    );
    base::sleep(0.03);
}
