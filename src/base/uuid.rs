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
    if status_code != 0 {
        base::log(&format!("Status code: {}", &status_code), 2);
    }

    while !(status_code == 200 || status_code == 0) {
        uuid = generate();
        base::log(
            &format!("Cheking uuid: {}-****-************", &uuid[..18]),
            2,
        );

        status_code = network::register(uuid.clone(), public_key.clone());

        if status_code != 0 {
            base::log(&format!("Status code: {}", &status_code), 2);
        }
    }

    if status_code == 0 {
        base::log("Failed to register", 1);
    }

    uuid
}

pub fn get() -> String {
    filesystem::cat(&filesystem::new_path("uuid"))
}
