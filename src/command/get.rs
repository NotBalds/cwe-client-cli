use crate::{base, modules};

pub fn run(passphrase: String) {
    let uuid = base::uuid::get();
    let gettime = base::unix_time().to_string();
    let gettimesignature = modules::crypting::sign(gettime.clone(), passphrase.clone());

    let (status_code, messages): (u16, modules::json::GetResponse) =
        modules::network::get(uuid.clone(), gettime, gettimesignature);
    base::log(&format!("Status code: {}", status_code), 3);

    for message in messages {
        println!(
            "{}",
            format!(
                "Message from {} - {}: \nContent: \n{}\n",
                base::contact::get_name(message.sender.clone()),
                message.sender,
                modules::crypting::decrypt(message.content, passphrase.clone())
            )
        );
    }
}
