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

    let mut iter = 0;
    for contact in contacts.clone() {
        base::log(
            &format!(
                "({}) {} - {}",
                iter,
                contact.clone(),
                base::contact::get(contact.clone()).0
            ),
            4,
        );
        iter += 1;
    }

    let mut contact_receiver = String::from("");

    loop {
        let contact_num = base::input("Enter number: ");
        if contact_num == "exit" {
            return;
        }
        match contact_num.parse::<i32>() {
            Ok(choice) => {
                if choice >= 0 && (choice as usize) < contacts.clone().len() {
                    contact_receiver.push_str(contacts[choice as usize].as_str());
                    break;
                } else {
                    base::log("Index out of bounds!", 3);
                }
            }
            Err(_) => base::log("Invalid number!", 3),
        };
    }

    let my_uuid = base::uuid::get();
    let (receiver_uuid, receiver_public_key) = base::contact::get(contact_receiver.clone());
    let message = base::input("Enter message: ");
    let sendtime = base::unix_time().to_string();
    let sendtimesignature = modules::crypting::sign(sendtime.clone(), passphrase.clone());

    modules::network::send(
        my_uuid,
        receiver_uuid,
        modules::crypting::encrypt(message.as_bytes().to_vec(), receiver_public_key),
        sendtime,
        sendtimesignature,
    );

    base::log("Message sent!", 0);
}
