use std::sync::{Arc, Mutex};

use log::info;
use pretty_env_logger;
use warp::Filter;

use tree_data_schema::{renderer_strings, Renderer, Renderers};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();
    let current_renderer = Arc::new(Mutex::new(Renderers::RedWave));

    let current_renderer_clone = current_renderer.clone();
    let renderers = warp::path!("renderers").map(move || {
        warp::reply::json({
            let strings = renderer_strings();

            info!("Getting renderers");

            // Find the one that is the current renderer and set to true
            &strings
                .iter()
                .map(|r| {
                    let mut renderer = r.clone();
                    if *current_renderer_clone.lock().unwrap() == renderer.name {
                        renderer.enabled = true;
                    }
                    renderer
                })
                .collect::<Vec<Renderer>>()
        })
    });

    // On the path renderer, accept a post request that changes the current
    // renderer
    let current_renderer_clone = current_renderer.clone();
    let renderer = warp::path!("renderer")
        .and(warp::post())
        .and(warp::body::json())
        .map(move |renderer: Renderers| {
            *current_renderer_clone.lock().unwrap() = renderer;
            info!("Current renderer is now {}", renderer);
            warp::reply::json(&renderer)
        });

    // And make a getter for the current renderer
    let current_renderer_clone = current_renderer.clone();
    let current_renderer = warp::path!("current_renderer").map(move || {
        info!("Getting current renderer");
        warp::reply::json(&*current_renderer_clone.lock().unwrap())
    });

    let index = warp::path::end().and(warp::fs::dir("/static"));

    let static_files = warp::path("static").and(warp::fs::dir("/static"));

    let routes = renderers
        .or(renderer)
        .or(current_renderer)
        .or(index)
        .or(static_files);

    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;

    Ok(())
}
