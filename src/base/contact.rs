use crate::{base, modules::json::Contact};

pub fn get(name: String) -> Contact {
    let contact_lines =
        base::filesystem::cat_lines(&base::filesystem::new_path("contacts").join(name.clone()));
    let uuid = contact_lines[0].clone();
    let mut public_key = String::from("");
    for line in contact_lines[1..].iter() {
        public_key.push_str(line);
        public_key.push('\n');
    }

    Contact {
        name,
        uuid,
        public_key,
    }
}

pub fn get_list() -> Vec<String> {
    base::filesystem::ls("contacts").unwrap()
}

pub fn get_name(uuid: String) -> String {
    if base::filesystem::exist(&format!("contacts-uuid/{}", uuid)) {
        base::filesystem::cat(&base::filesystem::new_path("contacts-uuid").join(uuid))
    } else {
        String::from("Unknown")
    }
}
