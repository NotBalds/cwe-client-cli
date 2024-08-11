use {
    crate::{
        base::{self, config, filesystem, uuid},
        modules::crypting,
    },
    std::io,
};

pub fn run(passphrase: String, force: bool) -> io::Result<()> {
    base::log("Preparing in process...", 2);

    // Preparing basic
    {
        // If Force mode -> remove old cwe-client directory
        if force {
            base::log("Deleting old cwe-clieny directory...", 2);

            filesystem::rmDirAll(&filesystem::new_path(""))
                .expect("Can't delete old cwe-client directory");

            base::log("Deleted old cwe-client directory", 0);
        }

        // Making cwe-client and cwe-client/base-keys directories
        base::log(
            "Making cwe-client and cwe-client/base-keys directories...",
            2,
        );

        filesystem::mkAllDirs(&filesystem::new_path("base-keys"))
            .expect("Can't create dir base-keys");

        base::log("Created cwe-client and cwe-client/base-keys directories", 0);
    }

    // Preparing RSA keys
    {
        // Generating rsa keys
        base::log("Generating rsa keys...", 2);

        let my_keys = crypting::rsa::gen_keys(passphrase.clone(), config::BASE_BITS);
        let my_private_key = my_keys.0;
        let my_public_key = my_keys.1;

        let sys_keys = crypting::rsa::gen_keys(passphrase.clone(), config::SYS_BITS);
        let sys_private_key = sys_keys.0;
        let sys_public_key = sys_keys.1;

        base::log("Generated rsa keys", 0);

        // Writing keys to directory base-keys
        base::log("Writing keys to directory base-keys...", 2);

        filesystem::echo(my_private_key, &filesystem::new_path("base-keys/my-key"));
        filesystem::echo(my_public_key, &filesystem::new_path("base-keys/my-key.pub"));

        filesystem::echo(sys_private_key, &filesystem::new_path("base-keys/sys-key"));
        filesystem::echo(
            sys_public_key.clone(),
            &filesystem::new_path("base-keys/sys-key.pub"),
        );

        base::log("Written keys to directory base-keys", 0);

        // Writing default server url to file
        base::log("Writing default server url to file...", 2);

        filesystem::echo(config::default_url(), &filesystem::new_path("server"));

        base::log("Written default server url to file", 0);
    }

    // Getting, registering and saving uuid
    {
        // Getting sys public key
        base::log("Getting sys public key...", 2);

        let sys_public_key = filesystem::cat(&filesystem::new_path("base-keys/sys-key.pub"));

        base::log("Got sys public key", 0);

        // Registration unused uuid
        base::log("Registration unused uuid...", 2);

        let uuid = uuid::register(sys_public_key);

        base::log("Registered unused uuid", 0);

        // Saving valid and unused uuid
        base::log("Saving valid and unused uuid...", 2);

        filesystem::echo(uuid, &filesystem::new_path("uuid"));

        base::log("Saved valid and unused uuid", 0);
    }

    // Last preparations
    {
        // Making directory contacts
        filesystem::mkAllDirs(&filesystem::new_path("contacts"))
            .expect("Can't create dir contacts");
        filesystem::mkAllDirs(&filesystem::new_path("contacts-uuid"))
            .expect("Can't create dir contacts");

        // Making message history directory
        filesystem::mkAllDirs(&filesystem::new_path("history")).expect("Can't create dir history");

        // Writing version to version file
        filesystem::echo(
            config::VERSION.to_string(),
            &filesystem::new_path("version"),
        );
    }

    base::log("Prepared cwe-client successfully", 0);

    Ok(())
}
