use crate::base;
use std::collections::HashMap;

pub fn run() {
    let commands: HashMap<&str, &str> = HashMap::from([
        ("help", "Show help"),
        ("exit", "Exit from the program"),
        ("send", "Send message"),
        ("get", "Get messages"),
        ("info", "Get my info"),
        ("import", "Import contact"),
        ("export", "Export my info"),
        ("del", "Delete contact"),
        ("clear", "Literally clear from UNIX consoles"),
        ("history", "Show history"),
    ]);

    base::log("List of commands:", 4);
    for (command_name, command_info) in &commands {
        base::log(&format!("|- {} - {}", command_name, command_info), 4);
        base::sleep(0.03);
    }
}
