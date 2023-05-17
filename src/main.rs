use geng::net::simple as simple_net;
use geng::prelude::*;

mod assets;
mod camera_torus;
mod game;
mod loading_screen;
mod main_menu;
mod model;
mod net;

use assets::*;
use net::*;

const TICKS_PER_SECOND: f32 = 20.0;

#[derive(clap::Parser, Clone)]
pub struct Opt {
    #[clap(long)]
    server: Option<String>,
    #[clap(long)]
    connect: Option<String>,
}

fn main() {
    logger::init().unwrap();

    let mut opt: Opt = program_args::parse();
    if opt.connect.is_none() && opt.server.is_none() {
        if cfg!(target_arch = "wasm32") {
            opt.connect = option_env!("CONNECT").map(|s| s.to_string());
            // .expect("Set CONNECT compile time env var")
            // .to_owned(),
        } else {
            opt.server = Some("127.0.0.1:1155".to_owned());
            opt.connect = Some("127.0.0.1:1155".to_owned());
        }
    }

    let model_constructor = {
        || {
            let path = static_path().join("server").join("config.json");
            let config = serde_json::from_reader(std::io::BufReader::new(
                std::fs::File::open(path).expect("Failed to server assets"),
            ))
            .expect("Failed to parse server assets");
            let assets = ServerAssets { config };
            model::Model::new(assets)
        }
    };
    let game_constructor = {
        let opt = opt.clone();
        move |geng: &Geng| {
            geng::LoadingScreen::new(
                geng,
                loading_screen::LoadingScreen::new(geng),
                {
                    let geng = geng.clone();
                    async move {
                        let common_path = static_path().join("shaders").join("common.glsl");
                        geng.shader_lib().add(
                            "common.glsl",
                            &<String as geng::LoadAsset>::load(&geng, &common_path)
                                .await
                                .context(format!(
                                    "Failed to load common.glsl from {:?}",
                                    common_path
                                ))?,
                        );

                        <Assets as geng::LoadAsset>::load(&geng, &static_path())
                            .then({
                                let geng = geng.clone();
                                move |assets| async move {
                                    match assets {
                                        Ok(mut assets) => {
                                            assets.process(&geng).await;
                                            Ok(assets)
                                        }
                                        Err(e) => Err(e),
                                    }
                                }
                            })
                            .await
                    }
                },
                {
                    let geng = geng.clone();
                    move |assets| {
                        let assets = assets.expect("Failed to load assets");
                        let assets = Rc::new(assets);
                        main_menu::MainMenu::new(&geng, &assets, opt)
                    }
                },
            )
        }
    };

    if opt.server.is_some() && opt.connect.is_none() {
        #[cfg(not(target_arch = "wasm32"))]
        simple_net::server::Server::new(opt.server.as_deref().unwrap(), model_constructor()).run();
    } else {
        #[cfg(not(target_arch = "wasm32"))]
        let server = if let Some(addr) = &opt.server {
            let server = simple_net::server::Server::new(addr, model_constructor());
            let server_handle = server.handle();
            let server_thread = std::thread::spawn(move || {
                server.run();
            });
            Some((server_handle, server_thread))
        } else {
            None
        };

        let geng = Geng::new_with(geng::ContextOptions {
            title: "Anlaut Summer Game Jam 2022".to_owned(),
            antialias: false,
            ..default()
        });
        let state = game_constructor(&geng);
        geng::run(&geng, state);

        #[cfg(not(target_arch = "wasm32"))]
        if let Some((server_handle, server_thread)) = server {
            server_handle.shutdown();
            server_thread.join().unwrap();
        }
    }
}
