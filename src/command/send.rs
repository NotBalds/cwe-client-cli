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

    if message_type == "Text" {
        send_text(passphrase, my_uuid, contact_receiver);
    }

    base::log("Message sent!", 0);
}

fn send_text(passphrase: String, my_uuid: String, receiver: modules::json::Contact) {
    let message = base::input("Enter message: ");

    let sendtime = base::unix_time().to_string();
    let sendtimesignature = modules::crypting::sign(sendtime.clone(), passphrase.clone());

    base::log("Sending...", 2);

    modules::network::send(
        my_uuid,
        receiver.uuid,
        modules::crypting::encrypt(message.as_bytes().to_vec(), receiver.public_key.clone()),
        modules::crypting::encrypt(b"text".to_vec(), receiver.public_key),
        sendtime,
        sendtimesignature,
    );
}
