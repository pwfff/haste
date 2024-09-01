use std::path::Path;

use flate2::read::GzDecoder;
use tar::Archive;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

macro_rules! proto_path {
    ($l:literal) => {
        concat!("GameTracking-Deadlock-master/Protobufs/", $l)
    };
}

fn main() -> Result<()> {
    let shared_protos = vec![
        proto_path!("demo.proto"),
        proto_path!("netmessages.proto"),
        proto_path!("network_connection.proto"),
        proto_path!("networkbasetypes.proto"),
        proto_path!("usermessages.proto"),
    ];

    #[cfg(feature = "dota2")]
    let dota2_protos = vec![
        proto_path!("dota_commonmessages.proto"),
        proto_path!("dota_shared_enums.proto"),
        proto_path!("dota_usermessages.proto"),
    ];

    #[cfg(feature = "deadlock")]
    let deadlock_protos = vec![
        proto_path!("citadel_clientmessages.proto"),
        proto_path!("citadel_gameevents.proto"),
        proto_path!("citadel_gamemessages.proto"),
        proto_path!("citadel_gcmessages_client.proto"),
        proto_path!("citadel_gcmessages_common.proto"),
        proto_path!("citadel_gcmessages_server.proto"),
        proto_path!("citadel_usercmd.proto"),
        proto_path!("citadel_usermessages.proto"),
        proto_path!("gcsdk_gcmessages.proto"),
        proto_path!("steammessages_steamlearn.steamworkssdk.proto"),
        proto_path!("steammessages_unified_base.steamworkssdk.proto"),
        proto_path!("steammessages.proto"),
        proto_path!("usercmd.proto"),
        proto_path!("gameevents.proto"),
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

    for p in &protos {
        println!("{p}");
    }

    println!("downloading latest archive...");
    let tar_gz = reqwest::blocking::get(
        "https://github.com/SteamDatabase/GameTracking-Deadlock/archive/master.tar.gz",
    )?;

    println!("archive downloaded, unpacking...");
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    for entry in archive.entries()? {
        let mut entry = entry?;
        let entry_path = entry.path()?;
        let tar_path_str = entry_path.to_str().unwrap();
        if !protos.contains(&tar_path_str) {
            continue;
        }
        let filename = Path::new(tar_path_str).file_name().unwrap();
        // unfortunately, i am not smart, and this only works when run from the haste workspace
        // root.
        let path = Path::new("./crates/haste_protos/protos").join(filename);
        println!("unpacking entry {tar_path_str} to {path:?}");
        entry.unpack(path)?;
    }

    Ok(())
}
