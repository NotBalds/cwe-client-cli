use {
    crate::{base, base::filesystem, modules::network},
    uuid::Uuid,
};

pub fn generate() -> String {
    let uuid = Uuid::new_v4().to_string();
    uuid
}

pub fn register(public_key: String) -> String {
    let mut uuid = generate();
    base::log(
        &format!("Cheking uuid: {}-****-************", &uuid[..18]),
        2,
    );

    let mut status_code = network::register(uuid.clone(), public_key.clone());
    base::log(&format!("Status code: {}", &status_code), 2);

    while !(status_code == 200) {
        uuid = generate();
        base::log(
            &format!("Cheking uuid: {}-****-************", &uuid[..18]),
            2,
        );

        status_code = network::register(uuid.clone(), public_key.clone());
        base::log(&format!("Status code: {}", &status_code), 2);
    }

    uuid
}

pub fn get() -> String {
    filesystem::cat(&filesystem::new_path("uuid"))
}
