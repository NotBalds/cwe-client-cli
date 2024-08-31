use crate::{base, modules::json::Contact};

pub fn get(name: String) -> Contact {
    let (contact_lines, uuid, public_key);
    if name == "Unknown" {
        uuid = String::from("");
        public_key = String::from("");
    } else {
        contact_lines =
            base::filesystem::cat_lines(&base::filesystem::new_path("contacts").join(name.clone()));

        uuid = contact_lines[0].clone();

        public_key = {
            let mut key = String::from("");
            for line in contact_lines[1..].iter() {
                key.push_str(line);
                key.push('\n');
            }
            key
        };
    }

    Contact {
        name,
        uuid,
        public_key,
    }
}

pub fn get_list() -> Vec<String> {
    base::filesystem::ls("contacts")
}

pub fn get_name(uuid: String) -> String {
    if base::filesystem::exist(&format!("contacts-uuid/{}", uuid)) {
        base::filesystem::cat(&base::filesystem::new_path("contacts-uuid").join(uuid))
    } else {
        String::from("Unknown")
    }
}

pub fn get_me() -> Contact {
    let name = String::from("Me");
    let uuid = base::filesystem::cat(&base::filesystem::new_path("uuid"));
    let public_key = String::from(base::filesystem::cat(&base::filesystem::new_path(
        "base-keys/my-key.pub",
    )));
    Contact {
        name,
        uuid,
        public_key,
    }
}
