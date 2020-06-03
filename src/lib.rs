#![type_length_limit = "2097152"]
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rmp_serde as rmps;

extern crate anyhow;
use anyhow::Result;
use std::collections::HashMap;

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
                argument: None
            })
        }
    }

    #[derive(Debug, Default, Deserialize, Serialize, Clone)]
    pub struct Command {
        pub cmd_type: CommandType,
        pub argument: Option<Arguments>
    }

    pub use bollard::container as bl;

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub enum Arguments {
        ListContainers {
            options: Option<ListContainersOptions<String>>
        },
        InspectContainer {
            name: String,
            options: Option<InspectContainerOptions>
        },
        ContainerChanges {
            name: String
        },
        Logs {
            name: String,
            options: Option<LogsOptions>
        },
        Stats {
            name: String,
            options: Option<StatsOptions>
        },
        Top {
            name: String,
            options: Option<TopOptions<String>>
        },
        Create {
            config: Config<String>,
            options: Option<CreateContainerOptions<String>>
        },
        Kill {
            name: String,
            options: Option<KillContainerOptions<String>>
        },
        Start {
            name: String,
            options: Option<StartContainerOptions<String>>
        },
        Stop {
            name: String,
            options: Option<StopContainerOptions>
        },
        Prune {
            options: Option<PruneContainersOptions<String>>
        },
        Remove {
            name: String,
            options: Option<RemoveContainerOptions>
        },
        Restart {
            name: String,
            options: Option<RestartContainerOptions>
        },
        Update {
            name: String,
            options: UpdateContainerOptions
        }

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
    pub enum Body {
        ContainerList(Vec<bollard::container::APIContainers>),
        Change(Option<Vec<bollard::container::Change>>),
        Container(bollard::container::Container),
        LogOutput(Vec<bollard::container::LogOutput>),
        Stats(Vec<bollard::container::Stats>),
        TopResult(bollard::container::TopResult),
        Error(String),
        PrunedContainers(bollard::container::PruneContainersResults),
        CreateContainerResults(bollard::container::CreateContainerResults),
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

        /*
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
        */

        pub async fn list() -> Result<Protocol> {
            Ok(Protocol {
                packet_type: Type::Command(Command { 
                    cmd_type: CommandType::List,
                    argument: Some(Arguments::ListContainers {
                        options: Some(bollard::container::ListContainersOptions {
                            all: true, 
                            ..Default::default()
                    })}) }),
                version: Protocol::cur_version(),
                client: Client::Docker,
                client_version: None,
                body: Body::Empty
            })
        }

        pub async fn list_opt(opt: Option<ListContainersOptions<String>>) -> Result<Protocol> {
            Ok(Protocol {
                packet_type: Type::Command(Command { 
                    cmd_type: CommandType::List,
                    argument: Some(Arguments::ListContainers {
                        options: opt}) }),
                version: Protocol::cur_version(),
                client: Client::Docker,
                client_version: None,
                body: Body::Empty
            })
        }



        pub async fn prune() -> Result<Protocol> {
            let mut filters = HashMap::new();
            filters.insert(String::from("until"), vec!(String::from("10m")));

            Ok(Protocol {
                packet_type: Type::Command(Command { 
                    cmd_type: CommandType::Prune,
                    argument: Some(Arguments::Prune {
                        options: Some(bollard::container::PruneContainersOptions {
                            filters: filters
                    })})}),
                version: Protocol::cur_version(),
                client: Client::Docker,
                client_version: None,
                body: Body::Empty
            })
        }

        pub async fn prune_opt(opt: Option<PruneContainersOptions<String>>) -> Result<Protocol> {
            Ok(Protocol {
                packet_type: Type::Command(Command { 
                    cmd_type: CommandType::Prune,
                    argument: Some(Arguments::Prune {
                        options: opt})}),
                version: Protocol::cur_version(),
                client: Client::Docker,
                client_version: None,
                body: Body::Empty
            })
        }

        pub async fn create() -> Result<Protocol> {
            let options = Some(bollard::container::CreateContainerOptions{
                name: String::from("my-new-container"),
            });
        
            let config = bollard::container::Config {
                image: Some(String::from("hello-world")),
                cmd: Some(vec![String::from("/hello")]),
                ..Default::default()
            };


            Ok(Protocol {
                packet_type: Type::Command(Command { 
                    cmd_type: CommandType::Create,
                    argument: Some(Arguments::Create {
                        options,
                        config
                    }) }),
                version: Protocol::cur_version(),
                client: Client::Docker,
                client_version: None,
                body: Body::Empty
            })
        }

        pub async fn create_opt(config: Config<String>, options: Option<CreateContainerOptions<String>>) -> Result<Protocol> {
            Ok(Protocol {
                packet_type: Type::Command(Command { 
                    cmd_type: CommandType::Create,
                    argument: Some(Arguments::Create {
                        options,
                        config
                    }) }),
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
                    argument: Some(Arguments::ContainerChanges {
                        name: String::from(name)
                    }) }),
                version: Protocol::cur_version(),
                client: Client::Docker,
                client_version: None,
                body: Body::Empty
            })
        }

        pub async fn change_opt(name: &str) -> Result<Protocol> {
            Ok(Protocol {
                packet_type: Type::Command(Command { 
                    cmd_type: CommandType::Change,
                    argument: Some(Arguments::ContainerChanges {
                        name: String::from(name)
                    }) }),
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
                    argument: Some(Arguments::InspectContainer {
                        name: String::from(name),
                        options: None::<bollard::container::InspectContainerOptions>
                    }) }),
                version: Protocol::cur_version(),
                client: Client::Docker,
                client_version: None,
                body: Body::Empty
            })
        }

        pub async fn inspect_opt(name: &str, opt: Option<InspectContainerOptions>) -> Result<Protocol> {
            Ok(Protocol {
                packet_type: Type::Command(Command { 
                    cmd_type: CommandType::Container,
                    argument: Some(Arguments::InspectContainer {
                        name: String::from(name),
                        options: opt
                    }) }),
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
                    argument: Some(Arguments::Logs {
                        name: String::from(name),
                        options: Some(bollard::container::LogsOptions {
                            follow: false,
                            stdout: true,
                            stderr: true,
                            tail: String::from("all"),
                            ..Default::default()
                        })
                    }) }),
                version: Protocol::cur_version(),
                client: Client::Docker,
                client_version: None,
                body: Body::Empty
            })
        }

        pub async fn logs_opt(name: &str, options: Option<LogsOptions>) -> Result<Protocol> {
            Ok(Protocol {
                packet_type: Type::Command(Command { 
                    cmd_type: CommandType::Log,
                    argument: Some(Arguments::Logs {
                        name: String::from(name),
                        options
                    }) }),
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
                    argument: Some(Arguments::Stats {
                        name: String::from(name),
                        options: Some(bollard::container::StatsOptions {
                            stream: false
                        })
                    }) }),
                version: Protocol::cur_version(),
                client: Client::Docker,
                client_version: None,
                body: Body::Empty
            })
        }

        pub async fn stats_opt(name: &str, options: Option<StatsOptions>) -> Result<Protocol> {
            Ok(Protocol {
                packet_type: Type::Command(Command { 
                    cmd_type: CommandType::Stats,
                    argument: Some(Arguments::Stats {
                        name: String::from(name),
                        options
                    }) }),
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
                    argument: Some(Arguments::Stop {
                        name: String::from(name),
                        options: Some(bollard::container::StopContainerOptions {
                            t: 32
                        })
                    }) }),
                version: Protocol::cur_version(),
                client: Client::Docker,
                client_version: None,
                body: Body::Empty
            })
        }

        pub async fn stop_opt(name: &str, options: Option<StopContainerOptions>) -> Result<Protocol> {
            Ok(Protocol {
                packet_type: Type::Command(Command { 
                    cmd_type: CommandType::Stop,
                    argument: Some(Arguments::Stop {
                        name: String::from(name),
                        options
                    }) }),
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
                    argument: Some(Arguments::Start {
                        name: String::from(name),
                        options: Some(bollard::container::StartContainerOptions {
                            detach_keys: "ctrl-^".into()
                        })
                    }) }),
                version: Protocol::cur_version(),
                client: Client::Docker,
                client_version: None,
                body: Body::Empty
            })
        }

        pub async fn start_opt(name: &str, options: Option<StartContainerOptions<String>>) -> Result<Protocol> {
            Ok(Protocol {
                packet_type: Type::Command(Command { 
                    cmd_type: CommandType::Start,
                    argument: Some(Arguments::Start {
                        name: String::from(name),
                        options
                    }) }),
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
                    argument: Some(Arguments::Kill {
                        name: name.into(),
                        options: Some(bollard::container::KillContainerOptions {
                            signal: String::from("SIGINT")
                        })
                    }) }),
                version: Protocol::cur_version(),
                client: Client::Docker,
                client_version: None,
                body: Body::Empty
            })
        }

        pub async fn kill_opt(name: &str, options: Option<KillContainerOptions<String>>) -> Result<Protocol> {
            Ok(Protocol {
                packet_type: Type::Command(Command { 
                    cmd_type: CommandType::Kill,
                    argument: Some(Arguments::Kill {
                        name: name.into(),
                        options
                    }) }),
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
                    argument: Some(Arguments::Restart {
                        name: name.into(),
                        options: Some(bollard::container::RestartContainerOptions {
                            t: 32
                        })
                    }) }),
                version: Protocol::cur_version(),
                client: Client::Docker,
                client_version: None,
                body: Body::Empty
            })
        }

        pub async fn restart_opt(name: &str, options: Option<RestartContainerOptions>) -> Result<Protocol> {
            Ok(Protocol {
                packet_type: Type::Command(Command { 
                    cmd_type: CommandType::Restart,
                    argument: Some(Arguments::Restart {
                        name: name.into(),
                        options
                    }) }),
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
                    argument: Some(Arguments::Top {
                        name: name.into(),
                        options: Some(bollard::container::TopOptions {
                            ps_args: "aux".into()
                        })
                    }) }),
                version: Protocol::cur_version(),
                client: Client::Docker,
                client_version: None,
                body: Body::Empty
            })
        }

        pub async fn top_opt(name: &str, options: Option<TopOptions<String>>) -> Result<Protocol> {
            Ok(Protocol {
                packet_type: Type::Command(Command { 
                    cmd_type: CommandType::Top,
                    argument: Some(Arguments::Top {
                        name: name.into(),
                        options
                    }) }),
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
                    argument: Some(Arguments::Remove {
                        name: name.into(),
                        options: Some(bollard::container::RemoveContainerOptions {
                            force: true,
                            ..Default::default()
                        })
                    }) }),
                version: Protocol::cur_version(),
                client: Client::Docker,
                client_version: None,
                body: Body::Empty
            })
        }

        pub async fn remove_opt(name: &str, options: Option<RemoveContainerOptions>) -> Result<Protocol> {
            Ok(Protocol {
                packet_type: Type::Command(Command { 
                    cmd_type: CommandType::Remove,
                    argument: Some(Arguments::Remove {
                        name: name.into(),
                        options
                    }) }),
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
                    argument: Some(Arguments::Update {
                        name: name.into(),
                        options: bollard::container::UpdateContainerOptions {
                            memory: Some(314572800),
                            memory_swap: Some(314572800),
                            ..Default::default()
                        }
                    })}),
                version: Protocol::cur_version(),
                client: Client::Docker,
                client_version: None,
                body: Body::Empty
            })
        }

        pub async fn update_opt(name: &str, options: UpdateContainerOptions) -> Result<Protocol> {
            Ok(Protocol {
                packet_type: Type::Command(Command { 
                    cmd_type: CommandType::Update,
                    argument: Some(Arguments::Update {
                        name: name.into(),
                        options
                    })}),
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