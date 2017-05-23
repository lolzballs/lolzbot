use mysql;
use serde_json;
use serenity;
use std;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        MySql(mysql::Error);
        SerdeJson(serde_json::Error);
        Serenity(serenity::Error);
    }

    errors {
        MutexPosioned {
            description("The mutex was poisoned")
        }
        NoDatabase {
            description("The database was not found")
        }
        UserError {
            description("A user requested an error")
            display("You wanted an error?")
        }
    }
}
