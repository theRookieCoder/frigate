use tokio::io::AsyncReadExt;

type NewModIdentifier = libium_new::config::structs::ModIdentifier;
type OldModLoaders = libium_old::config::structs::ModLoaders;
type NewModLoaders = libium_new::config::structs::ModLoaders;
type NewProfile = libium_new::config::structs::Profile;
type OldConfig = libium_old::config::structs::Config;
type NewConfig = libium_new::config::structs::Config;
type OldMod = libium_old::config::structs::Mod;
type NewMod = libium_new::config::structs::Mod;

#[tokio::main]
async fn main() {
    let mut config_file =
        libium_old::config::get_config_file(libium_old::config::config_file_path())
            .await
            .expect("Could not open config file");
    let mut config_file_contents = String::new();
    config_file
        .read_to_string(&mut config_file_contents)
        .await
        .expect("Could not read config file");
    let old_config: OldConfig =
        serde_json::from_str(&config_file_contents).expect("Could not deserialise config file");

    let mut new_config = NewConfig {
        active_profile: old_config.active_profile,
        profiles: Vec::new(),
    };

    for old_profile in old_config.profiles {
        let mut new_profile = NewProfile {
            name: old_profile.name,
            output_dir: old_profile.output_dir,
            game_version: old_profile.game_version,
            mod_loader: match old_profile.mod_loader {
                OldModLoaders::Fabric => NewModLoaders::Fabric,
                OldModLoaders::Forge => NewModLoaders::Forge,
            },
            mods: Vec::new(),
        };

        for old_mod in old_profile.mods {
            let new_mod = NewMod {
                name: old_mod.name().into(),
                identifier: match old_mod {
                    OldMod::CurseForgeProject { project_id, .. } => {
                        NewModIdentifier::CurseForgeProject(project_id)
                    }
                    OldMod::ModrinthProject { project_id, .. } => {
                        NewModIdentifier::ModrinthProject(project_id)
                    }
                    OldMod::GitHubRepository { full_name, .. } => {
                        NewModIdentifier::GitHubRepository(full_name)
                    }
                },
            };
            new_profile.mods.push(new_mod);
        }

        new_config.profiles.push(new_profile);
    }

    libium_new::config::write_config(&mut config_file, &new_config)
        .await
        .expect("Could not write config");
}
