use crate::{base, modules};

pub fn run(passphrase: String) {
    base::log("Please select contact name: ", 5);

    let contacts = match base::filesystem::ls("contacts") {
        Ok(contacts) => contacts,
        Err(err) => {
            base::log(&format!("Error: {}", err), 1);
            vec![]
        }
    };

    let contact_receiver =
        contacts[base::get_choice(contacts.clone(), "Select contact: ") as usize].clone();

    let my_uuid = base::uuid::get();
    let contact_receiver = base::contact::get(contact_receiver.clone());

    let supported_types: Vec<String> = modules::config::supported_types();
    let message_type = supported_types
        [base::get_choice(supported_types.clone(), "Select message type: ") as usize]
        .clone();

    base::log(&message_type, 3);

    let sending_data: Vec<u8>;

    match message_type.as_str() {
        "Text" => {
            sending_data = base::input("Enter message: ").as_bytes().to_vec();
        }
        "Image" => {
            let path_to_img = base::correct_input(
                "Enter ABSOLUTE path to image: ",
                base::filesystem::exist_abs,
            );
            sending_data = base::filesystem::bcat(path_to_img);
        }
        _ => {
            base::log("Unknown message type", 1);
            sending_data = vec![];
        }
    }

    let sendtime = base::unix_time().to_string();
    let sendtimesignature = modules::crypting::sign(sendtime.clone(), passphrase.clone());

    base::log("Sending...", 2);

    modules::network::send(
        my_uuid,
        contact_receiver.uuid,
        modules::crypting::encrypt_data(sending_data, contact_receiver.public_key.clone()),
        modules::crypting::encrypt_data(
            message_type.as_bytes().to_vec(),
            contact_receiver.public_key,
        ),
        sendtime,
        sendtimesignature,
    );

    base::log("Message sent!", 0);
}
