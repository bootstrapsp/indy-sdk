#[macro_use]
extern crate log;

mod commands;
mod services;

use commands::{Command, CommandExecutor};
use std::error;

pub struct SovrinClient {
    command_executor: CommandExecutor
}

impl SovrinClient {
    pub fn new() -> SovrinClient {
        SovrinClient {
            command_executor: CommandExecutor::new()
        }
    }

    pub fn set_did(&self, did: String, cb: Box<Fn(Result<(), Box<error::Error>>) + Send>) {
        self.command_executor.send(Command::SetDidCommand(did, cb));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc::channel;

    #[test]
    fn sovrin_client_can_be_created() {
        let sovrin_client = SovrinClient::new();
        assert!(true, "No crashes on SovrinClient::new");
    }

    #[test]
    fn sovrin_client_can_be_dropped() {
        fn drop_test() {
            let sovrin_client = SovrinClient::new();
        }

        drop_test();
        assert!(true, "No crashes on SovrinClient::drop");
    }

    #[test]
    fn set_did_method_can_be_called() {
        let (sender, receiver) = channel();

        let cb = Box::new(move |result| {
            match result {
                Ok(val) => sender.send("OK"),
                Err(err) => sender.send("ERROR")
            };
        });

        let sovrin_client = SovrinClient::new();
        sovrin_client.set_did("DID0".to_string(), cb);

        match receiver.recv() {
            Ok(result) => {
                assert_eq!("OK", result);
            }
            Err(err) => {
                panic!("Error on result recv: {:?}", err);
            }
        }
    }
}