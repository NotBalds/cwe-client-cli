use crate::base;

pub fn run() -> i8 {
    base::log("Checking existence of necessary files", 2);
    if base::filesystem::exist("") {
        if !(base::filesystem::exist("uuid")
            && base::filesystem::exist("base-keys/sys-key")
            && base::filesystem::exist("base-keys/sys-key.pub")
            && base::filesystem::exist("base-keys/my-key")
            && base::filesystem::exist("base-keys/my-key.pub"))
        {
            base::log(
                "Cannot find necessary files. (~/.local/share/cwe-client)",
                3,
            );
            if !(base::filesystem::exist("uuid")) {
                base::log("Missing uuid file", 3);
            }
            if !(base::filesystem::exist("base-keys/sys-key")
                || base::filesystem::exist("base-keys/sys-key.pub"))
            {
                base::log("Missing sys-key files", 3);
            }
            if !(base::filesystem::exist("base-keys/my-key")
                || base::filesystem::exist("base-keys/my-key.pub"))
            {
                base::log("Missing my-key files", 3);
            }
            2
        } else {
            base::log("Files found", 0);
            let current_version: String = base::config::VERSION.to_string();
            let files_version: String =
                base::filesystem::cat(&base::filesystem::new_path("version"));
            if files_version != current_version {
                base::log(
                    &format!(
                        "Mismatched version. App version: {}. Found files version: {}.",
                        current_version, files_version
                    ),
                    3,
                );
                3
            } else {
                0
            }
        }
    } else {
        1
    }
}
