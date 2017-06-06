use hyper;
use mysql;
use serde_json;
use serenity;
use std;

error_chain! {
    foreign_links {
        Hyper(hyper::Error);
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
        NoBotId {
            description("There was no bot id")
        }
        UserError {
            description("A user requested an error")
            display("You wanted an error?")
        }
    }
}
