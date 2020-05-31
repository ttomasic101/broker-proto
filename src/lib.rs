#![type_length_limit = "2097152"]
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rmp_serde as rmps;

#[allow(unused_imports)]
use bollard::container::*;



    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[allow(unused_imports)]
    use rmps::{Deserializer, Serializer};

    #[derive(Debug, Default, Deserialize, Serialize)]
    pub struct Protocol {
        version: Version,
        client: Client,
        client_version: Option<Version>,
        body: Body,
        packet_type: Type,
    }

    #[derive(Debug, PartialEq, Default, Deserialize, Serialize)]
    pub struct Version {
        major: u64,
        minor: u64,
        patch: u64,
    }

    #[derive(Debug, Deserialize, Serialize)]
    enum Type { Command(Command), Response, Transfer}

    impl Default for Type {
        fn default() -> Self {
            Type::Command(Command { cmd_type: CommandType::List})
        }
    }

    #[derive(Debug, Default, Deserialize, Serialize)]
    struct Command {
        cmd_type: CommandType,
        
    }

    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    pub enum CommandType { List, Change, Container, Log, Stats, Top, }

    impl Default for CommandType {
        fn default() -> Self {
            CommandType::List
        }
    }

    impl Version {
        fn new(major: u64, minor: u64, patch: u64) -> Self {
            Version { major, minor, patch}
        }
    }

    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    enum Client { Docker, Lxc }

    impl Default for Client {
        fn default() -> Self {
            Client::Docker
        }
    }

    #[derive(Debug, Deserialize, Serialize)]
    struct ContainerList(bollard::container::APIContainers);

    #[derive(Debug, Deserialize, Serialize)]
    struct Change(bollard::container::Change);

    #[derive(Debug, Deserialize, Serialize)]
    struct Container(bollard::container::Container);

    //#[derive(Debug, Deserialize, Serialize)]
    //struct LogOutput(bollard::container::LogOutput);

    #[derive(Debug, Deserialize, Serialize)]
    struct Stats(bollard::container::Stats);

    #[derive(Debug, Deserialize, Serialize)]
    struct TopResult(bollard::container::TopResult);

    #[derive(Debug, Deserialize, Serialize)]
    #[serde(remote = "bollard::container::LogOutput")]
    enum LogOutputWrapper {
        StdErr {
            message: String,
        },
        StdOut {
            message: String,
        },
        StdIn {
            message: String,
        },
        Console {
            message: String,
        },
    }

    #[derive(Debug, Deserialize, Serialize)]
    enum Body {
        ContainerList(ContainerList),
        Change(Change),
        Container(Container),
        #[serde(with = "LogOutputWrapper")]
        LogOutput(bollard::container::LogOutput),
        Stats(Stats),
        TopResult(TopResult),
        Empty,
    }

    impl Default for Body {
        fn default() -> Self {
            Body::Empty
        }
    }

    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    struct Empty {}

    impl Protocol {
        pub fn new() -> Self {
            Protocol {
                version: Version::new(1, 0, 0),
                client: Client::Docker,
                client_version: Some(Protocol::cur_version()),
                body: Body::new(),
                packet_type: Default::default(),
            }
        }

        pub fn cur_version() -> Version {
            Version::new(1, 0, 0)
        }

        pub fn bytes(self: &Self) -> Result<Vec<u8>, rmps::encode::Error> {
            let mut buf = Vec::new();

            self.serialize(&mut Serializer::new(&mut buf))?;

            Ok(buf)
        }

        pub fn command(cmd: CommandType) -> Protocol {
            Protocol {
                packet_type: Type::Command(Command { cmd_type: cmd}),
                ..Default::default()
            }
        }
    }

    use std::convert::TryFrom;

    impl TryFrom<&[u8]> for Protocol {
        type Error = rmps::decode::Error;

        fn try_from(item: &[u8]) -> Result<Self, Self::Error> {
            use std::io::Cursor;

            let cursor = Cursor::new(item);
            let mut de = Deserializer::new(cursor);

            Deserialize::deserialize(&mut de)
        }
    }

    impl Body {
        fn new() -> Self {
            Body::Empty
        }
    }

    impl TryFrom<Protocol> for Vec<u8> {
        type Error = rmps::encode::Error;

        fn try_from(item: Protocol) -> Result<Self, Self::Error> {
            let mut buf = Vec::new();

            item.serialize(&mut Serializer::new(&mut buf))?;

            Ok(buf)
        }
    }