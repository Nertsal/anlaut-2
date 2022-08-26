use geng::net::simple as net;
use geng::prelude::*;

mod assets;
mod game;
mod loading_screen;
mod model;

use assets::*;

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
            opt.connect = Some(
                option_env!("CONNECT")
                    .expect("Set CONNECT compile time env var")
                    .to_owned(),
            );
        } else {
            opt.server = Some("127.0.0.1:1155".to_owned());
            opt.connect = Some("127.0.0.1:1155".to_owned());
        }
    }

    let model_constructor = {
        || {
            let path = static_path().join("server.json");
            let assets = serde_json::from_reader(std::io::BufReader::new(
                std::fs::File::open(path).expect("Failed to server assets"),
            ))
            .expect("Failed to parse server assets");
            model::Model::new(assets)
        }
    };
    let game_constructor = {
        // let opt = opt.clone();
        move |geng: &Geng, player_id, model| {
            geng::LoadingScreen::new(
                geng,
                loading_screen::LoadingScreen::new(geng),
                <Assets as geng::LoadAsset>::load(geng, &static_path()).then({
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
                }),
                {
                    let geng = geng.clone();
                    move |assets| {
                        let assets = assets.expect("Failed to load assets");
                        // client::run(&geng, &Rc::new(assets), player_id, &opt, model)
                        game::Game::new(&geng, &Rc::new(assets), player_id, model)
                    }
                },
            )
        }
    };

    if opt.server.is_some() && opt.connect.is_none() {
        #[cfg(not(target_arch = "wasm32"))]
        net::server::Server::new(opt.server.as_deref().unwrap(), model_constructor()).run();
    } else {
        #[cfg(not(target_arch = "wasm32"))]
        let server = if let Some(addr) = &opt.server {
            let server = net::server::Server::new(addr, model_constructor());
            let server_handle = server.handle();
            let server_thread = std::thread::spawn(move || {
                server.run();
            });
            Some((server_handle, server_thread))
        } else {
            None
        };

        let geng = Geng::new_with(geng::ContextOptions {
            title: "Extremely Extreme Sports".to_owned(),
            antialias: false,
            ..default()
        });
        let state = net::ConnectingState::new(&geng, opt.connect.as_deref().unwrap(), {
            let geng = geng.clone();
            move |player_id, model| game_constructor(&geng, player_id, model)
        });
        geng::run(&geng, state);

        #[cfg(not(target_arch = "wasm32"))]
        if let Some((server_handle, server_thread)) = server {
            server_handle.shutdown();
            server_thread.join().unwrap();
        }
    }
}
