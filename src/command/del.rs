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

    let selected_contact = base::contact::get(
        contacts[base::get_choice(contacts.clone(), "Select contact: ") as usize].clone(),
    );

    base::log("Deleting contact...", 2);

    base::filesystem::del_file(&format!("contacts/{}", selected_contact.name));
    base::filesystem::del_file(&format!("contacts-uuid/{}", selected_contact.uuid));

    base::log("Contact deleted successfully!", 0);
}
