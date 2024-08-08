use crate::{
    base,
    modules::{self, crypting, json},
};

pub fn run(passphrase: String) {
    base::log("Please select contact name: ", 5);

    let contacts = match base::filesystem::ls("contacts") {
        Ok(contacts) => contacts,
        Err(err) => {
            base::log(&format!("Error: {}", err), 1);
            vec![]
        }
    };

    let user_choice = base::get_choice(contacts.clone(), "Select contact: ");
    if user_choice == -1 {
        return;
    }

    let contact_receiver = contacts[user_choice as usize].clone();

    let sender = base::uuid::get();
    let contact_receiver = base::contact::get(contact_receiver.clone());

    let supported_types: Vec<String> = modules::config::supported_types();
    let message_type = supported_types
        [base::get_choice(supported_types.clone(), "Select message type: ") as usize]
        .clone();

    base::log(&message_type, 3);

    let (content_format, (content_info, content_data_blocks)): (
        String,
        ((String, u128), Vec<String>),
    );
    let data: Vec<u8>;

    match message_type.as_str() {
        "Text" => {
            data = base::input("Enter message: ").as_bytes().to_vec();
        }
        "Image" => {
            let path_to_img = base::correct_input(
                "Enter ABSOLUTE path to image: ",
                base::filesystem::exist_abs,
            );
            data = base::filesystem::bcat(path_to_img);
        }
        _ => {
            base::log("Unknown message type", 1);
            data = vec![];
        }
    }

    base::log("Sending...", 2);

    (content_info, content_data_blocks) =
        modules::crypting::encrypt_data(&data, contact_receiver.public_key.clone()).clone();
    content_format = crypting::encrypt_data(
        message_type.clone().as_bytes(),
        contact_receiver.public_key.clone(),
    )
    .1[0]
        .clone();

    let content_info =
        content_info.0.to_string().as_str().to_owned() + "|" + content_info.1.to_string().as_str();
    let content_info =
        crypting::encrypt_data(content_info.as_bytes(), contact_receiver.public_key.clone()).1[0]
            .clone();

    let mut current_data_block = 0;
    for content_data_block in content_data_blocks.clone() {
        current_data_block += 1;
        let message = json::Message {
            sender: sender.clone(),
            content: json::Content {
                format: content_format.clone(),
                info: content_info.clone(),
                data: content_data_block.clone(),
            },
        };

        let sendtime = base::unix_time().to_string();
        let sendtimesignature = modules::crypting::sign(sendtime.clone(), passphrase.clone());

        let status_code = modules::network::send(
            contact_receiver.uuid.clone(),
            message,
            sendtime,
            sendtimesignature,
        );

        if status_code != 200 {
            base::log(&format!("Status code: {}", status_code), 3);
            base::log("Message not sent!", 3);
            base::log("Error got from server", 4);
            return;
        } else {
            base::log(
                &format!(
                    "Sent block {}/{}",
                    current_data_block,
                    content_data_blocks.len().clone()
                ),
                0,
            );
        }
    }

    base::log("Message sent!", 0);
}
