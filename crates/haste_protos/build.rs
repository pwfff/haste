fn main() -> std::io::Result<()> {
    std::env::set_var("PROTOC", protobuf_src::protoc());

    let shared_protos = vec![
        "protos/demo.proto",
        "protos/netmessages.proto",
        "protos/network_connection.proto",
        "protos/networkbasetypes.proto",
        "protos/usermessages.proto",
        #[cfg(feature = "deadlock")]
        "protos/gcsdk_gcmessages.proto",
        #[cfg(feature = "deadlock")]
        "protos/citadel_gcmessages_common.proto",
        #[cfg(feature = "deadlock")]
        "protos/steammessages_steamlearn.steamworkssdk.proto",
        #[cfg(feature = "deadlock")]
        "protos/steammessages_unified_base.steamworkssdk.proto",
        #[cfg(feature = "deadlock")]
        "protos/steammessages.proto",
        #[cfg(feature = "deadlock")]
        "protos/usercmd.proto",
        #[cfg(feature = "deadlock")]
        "protos/gameevents.proto",
    ];

    #[cfg(feature = "dota2")]
    let dota2_protos = vec![
        "dota_commonmessages.proto",
        "dota_shared_enums.proto",
        "dota_usermessages.proto",
    ];

    #[cfg(feature = "deadlock")]
    let deadlock_protos = vec![
        "citadel_clientmessages.proto",
        "citadel_gameevents.proto",
        "citadel_gamemessages.proto",
        "citadel_gcmessages_client.proto",
        "citadel_gcmessages_server.proto",
        "citadel_usercmd.proto",
        "citadel_usermessages.proto",
    ];

    let protos = vec![
        shared_protos,
        #[cfg(feature = "dota2")]
        dota2_protos,
        #[cfg(feature = "deadlock")]
        deadlock_protos,
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<&str>>();

    let includes = ["protos"];

    prost_build::compile_protos(&protos, &includes)
}
