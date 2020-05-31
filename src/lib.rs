extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rmp_serde as rmps;



    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[allow(unused_imports)]
    use rmps::{Deserializer, Serializer};

    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    pub struct Protocol {
        version: Version,
        client: Client,
        client_version: Option<Version>,
        body: Body,
        packet_type: Type,
    }

    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    pub struct Version {
        major: u64,
        minor: u64,
        patch: u64,
    }

    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    struct Command {
        cmd_type: CommandType,
        
    }

    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    enum CommandType { List, Info, Update, Modify, Stop, Delete, New }

    impl Version {
        fn new(major: u64, minor: u64, patch: u64) -> Self {
            Version { major, minor, patch}
        }
    }

    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    enum Client { Docker, Lxc }

    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    enum Type { Command, Response, Transfer}

    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    struct Body {}

    impl Protocol {
        pub fn new() -> Self {
            Protocol {
                version: Version::new(1, 0, 0),
                client: Client::Docker,
                client_version: Some(Protocol::cur_version()),
                body: Body::new(),
                packet_type: Type::Command,
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
            Body {}
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