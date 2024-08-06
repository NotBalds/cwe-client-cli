use crate::base;

pub fn run() {
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

    let mut selected_contact = String::from("");

    loop {
        let contact_num = base::input("Enter number: ");
        if contact_num == "exit" {
            return;
        }
        match contact_num.parse::<i32>() {
            Ok(choice) => {
                if choice >= 0 && (choice as usize) < contacts.clone().len() {
                    selected_contact.push_str(contacts[choice as usize].as_str());
                    break;
                } else {
                    base::log("Index out of bounds!", 3);
                }
            }
            Err(_) => base::log("Invalid number!", 3),
        };
    }

    let selected_contact_uuid = base::contact::get(selected_contact.clone()).0;

    base::log("Deleting contact...", 2);

    base::filesystem::del_file(&format!("contacts/{}", selected_contact));
    base::filesystem::del_file(&format!("contacts-uuid/{}", selected_contact_uuid));

    base::log("Contact deleted successfully!", 0);
}
