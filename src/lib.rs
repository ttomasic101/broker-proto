#![type_length_limit = "2097152"]
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rmp_serde as rmps;

extern crate anyhow;
use anyhow::Result;

#[allow(unused_imports)]
use bollard::container::*;



    #[allow(unused_imports)]
    use serde::{Deserialize, Serialize};
    #[allow(unused_imports)]
    use rmps::{Deserializer, Serializer};

    use bollard::Docker;

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Protocol {
        pub version: Version,
        pub client: Client,
        pub client_version: Option<ClientVersion>,
        pub body: Body,
        pub packet_type: Type,
    }

    #[derive(Debug, PartialEq, Deserialize, Serialize)]
    pub struct Version {
        major: u64,
        minor: u64,
        patch: u64,
    }

    type ClientVersion = bollard::system::Version;

    #[derive(Debug, Deserialize, Serialize)]
    pub enum Type { Command(Command), Response, Transfer, Other}

    impl Default for Type {
        fn default() -> Self {
            Type::Command(Command { 
                cmd_type: CommandType::List,
                name: None
            })
        }
    }

    #[derive(Debug, Default, Deserialize, Serialize, Clone)]
    pub struct Command {
        pub cmd_type: CommandType,
        pub name: Option<String>,
        
    }

    #[derive(Debug, PartialEq, Deserialize, Serialize, Copy, Clone)]
    pub enum CommandType { List, Change, Container, Log, Stats, Top, Create, Kill, Start, Stop, Prune, Remove, Restart, Update, Wait}

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
    pub enum Client { Docker, Lxc }

    impl Default for Client {
        fn default() -> Self {
            Client::Docker
        }
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct ContainerList(pub bollard::container::APIContainers);

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Change(pub bollard::container::Change);

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Container(pub bollard::container::Container);

    //#[derive(Debug, Deserialize, Serialize)]
    //struct LogOutput(bollard::container::LogOutput);

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Stats(pub bollard::container::Stats);

    #[derive(Debug, Deserialize, Serialize)]
    pub struct TopResult(pub bollard::container::TopResult);

    #[derive(Debug, Deserialize, Serialize)]
    pub struct PruneContainerResults(pub bollard::container::PruneContainersResults);

    #[derive(Debug, Deserialize, Serialize)]
    pub struct CreateContainerResults(pub bollard::container::CreateContainerResults);

    #[derive(Debug, Deserialize, Serialize)]
    pub enum LogOutputWrapper {
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
    pub enum Body {
        ContainerList(Vec<ContainerList>),
        Change(Option<Vec<Change>>),
        Container(Container),
        LogOutput(Vec<LogOutputWrapper>),
        Stats(Vec<Stats>),
        TopResult(TopResult),
        Error(String),
        PrunedContainers(PruneContainerResults),
        CreateContainerResults(CreateContainerResults),
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
        /*
        pub fn new() -> Self {
            Protocol {
                version: Version::new(1, 0, 0),
                client: Client::Docker,
                client_version: Some(Protocol::cur_version()),
                body: Body::new(),
                packet_type: Default::default(),
            }
        }
        */

        pub fn cur_version() -> Version {
            Version::new(1, 0, 0)
        }

        pub fn bytes(self: &Self) -> Result<Vec<u8>, rmps::encode::Error> {
            let mut buf = Vec::new();

            self.serialize(&mut Serializer::new(&mut buf))?;

            Ok(buf)
        }

        pub async fn command(cmd: CommandType) -> Result<Protocol> {
            Ok(Protocol {
                packet_type: Type::Command(Command { 
                    cmd_type: cmd,
                    name: None }),
                version: Protocol::cur_version(),
                client: Client::Docker,
                client_version: None,
                body: Body::Empty
            })
        }

        pub async fn list() -> Result<Protocol> {
            Ok(Protocol {
                packet_type: Type::Command(Command { 
                    cmd_type: CommandType::List,
                    name: None }),
                version: Protocol::cur_version(),
                client: Client::Docker,
                client_version: None,
                body: Body::Empty
            })
        }

        pub async fn prune() -> Result<Protocol> {
            Ok(Protocol {
                packet_type: Type::Command(Command { 
                    cmd_type: CommandType::Prune,
                    name: None }),
                version: Protocol::cur_version(),
                client: Client::Docker,
                client_version: None,
                body: Body::Empty
            })
        }

        pub async fn create() -> Result<Protocol> {
            Ok(Protocol {
                packet_type: Type::Command(Command { 
                    cmd_type: CommandType::Create,
                    name: None }),
                version: Protocol::cur_version(),
                client: Client::Docker,
                client_version: None,
                body: Body::Empty
            })
        }

        pub async fn change(name: &str) -> Result<Protocol> {
            Ok(Protocol {
                packet_type: Type::Command(Command { 
                    cmd_type: CommandType::Change,
                    name: Some(String::from(name)) }),
                version: Protocol::cur_version(),
                client: Client::Docker,
                client_version: None,
                body: Body::Empty
            })
        }

        pub async fn inspect(name: &str) -> Result<Protocol> {
            Ok(Protocol {
                packet_type: Type::Command(Command { 
                    cmd_type: CommandType::Container,
                    name: Some(String::from(name)) }),
                version: Protocol::cur_version(),
                client: Client::Docker,
                client_version: None,
                body: Body::Empty
            })
        }

        pub async fn logs(name: &str) -> Result<Protocol> {
            Ok(Protocol {
                packet_type: Type::Command(Command { 
                    cmd_type: CommandType::Log,
                    name: Some(String::from(name)) }),
                version: Protocol::cur_version(),
                client: Client::Docker,
                client_version: None,
                body: Body::Empty
            })
        }

        pub async fn stats(name: &str) -> Result<Protocol> {
            Ok(Protocol {
                packet_type: Type::Command(Command { 
                    cmd_type: CommandType::Stats,
                    name: Some(String::from(name)) }),
                version: Protocol::cur_version(),
                client: Client::Docker,
                client_version: None,
                body: Body::Empty
            })
        }

        pub async fn stop(name: &str) -> Result<Protocol> {
            Ok(Protocol {
                packet_type: Type::Command(Command { 
                    cmd_type: CommandType::Stop,
                    name: Some(String::from(name)) }),
                version: Protocol::cur_version(),
                client: Client::Docker,
                client_version: None,
                body: Body::Empty
            })
        }

        pub async fn start(name: &str) -> Result<Protocol> {
            Ok(Protocol {
                packet_type: Type::Command(Command { 
                    cmd_type: CommandType::Start,
                    name: Some(String::from(name)) }),
                version: Protocol::cur_version(),
                client: Client::Docker,
                client_version: None,
                body: Body::Empty
            })
        }

        pub async fn kill(name: &str) -> Result<Protocol> {
            Ok(Protocol {
                packet_type: Type::Command(Command { 
                    cmd_type: CommandType::Kill,
                    name: Some(String::from(name)) }),
                version: Protocol::cur_version(),
                client: Client::Docker,
                client_version: None,
                body: Body::Empty
            })
        }

        pub async fn restart(name: &str) -> Result<Protocol> {
            Ok(Protocol {
                packet_type: Type::Command(Command { 
                    cmd_type: CommandType::Restart,
                    name: Some(String::from(name)) }),
                version: Protocol::cur_version(),
                client: Client::Docker,
                client_version: None,
                body: Body::Empty
            })
        }

        pub async fn top(name: &str) -> Result<Protocol> {
            Ok(Protocol {
                packet_type: Type::Command(Command { 
                    cmd_type: CommandType::Top,
                    name: Some(String::from(name)) }),
                version: Protocol::cur_version(),
                client: Client::Docker,
                client_version: None,
                body: Body::Empty
            })
        }

        pub async fn remove(name: &str) -> Result<Protocol> {
            Ok(Protocol {
                packet_type: Type::Command(Command { 
                    cmd_type: CommandType::Remove,
                    name: Some(String::from(name)) }),
                version: Protocol::cur_version(),
                client: Client::Docker,
                client_version: None,
                body: Body::Empty
            })
        }

        pub async fn update(name: &str) -> Result<Protocol> {
            Ok(Protocol {
                packet_type: Type::Command(Command { 
                    cmd_type: CommandType::Update,
                    name: Some(String::from(name)) }),
                version: Protocol::cur_version(),
                client: Client::Docker,
                client_version: None,
                body: Body::Empty
            })
        }

        pub async fn response(docker: &Docker) -> Result<Protocol> {
            Ok(Protocol {
                packet_type: Type::Response,
                version: Protocol::cur_version(),
                client: Client::Docker,
                client_version: Some(docker.version().await?),
                body: Body::Empty
            })
        }

        pub async fn error(docker: &Docker, err: &str) -> Result<Protocol> {
            Ok(Protocol {
                packet_type: Type::Response,
                version: Protocol::cur_version(),
                client: Client::Docker,
                client_version: Some(docker.version().await?),
                body: Body::Error(String::from(err))
            })
        }

        pub fn error_none(err: &str) -> Protocol {
            Protocol {
                packet_type: Type::Response,
                version: Protocol::cur_version(),
                client: Client::Docker,
                client_version: None,
                body: Body::Error(String::from(err))
            }
        }

        pub async fn default(docker: &Docker) -> Result<Protocol> {
            Ok(Protocol {
                packet_type: Type::Other,
                version: Protocol::cur_version(),
                client_version: Some(docker.version().await?),
                client: Client::Docker,
                body: Body::Empty
            })
        }

        pub async fn default_none() -> Result<Protocol> {
            Ok( Protocol {
                packet_type: Type::Other,
                version: Protocol::cur_version(),
                client_version: None,
                client: Client::Docker,
                body: Body::Empty
            })
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

    impl TryFrom<Protocol> for Vec<u8> {
        type Error = rmps::encode::Error;

        fn try_from(item: Protocol) -> Result<Self, Self::Error> {
            let mut buf = Vec::new();

            item.serialize(&mut Serializer::new(&mut buf))?;

            Ok(buf)
        }
    }