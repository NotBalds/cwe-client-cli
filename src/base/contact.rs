use crate::base;

pub fn get(name: String) -> (String, String) {
    let contact_lines =
        base::filesystem::cat_lines(&base::filesystem::new_path("contacts").join(name));
    let contact_uuid = contact_lines[0].clone();
    let mut contact_public_key = String::from("");
    for line in contact_lines[1..].iter() {
        contact_public_key.push_str(line);
        contact_public_key.push('\n');
    }

    (contact_uuid, contact_public_key)
}

pub fn get_name(uuid: String) -> String {
    if base::filesystem::exist(&format!("contacts-uuid/{}", uuid)) {
        base::filesystem::cat(&base::filesystem::new_path("contacts-uuid").join(uuid))
    } else {
        String::from("Unknown")
    }
}
