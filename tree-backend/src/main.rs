use std::sync::{Arc, Mutex};

use warp::Filter;

use tree_data_schema::{renderer_strings, Renderer, RendererStrings, Renderers};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let current_renderer = Arc::new(Mutex::new(Renderers::RedLines));

    let renderers = warp::path!("renderers").map(move || {
        warp::reply::json({
            let strings = renderer_strings();

            // Find the one that is the current renderer and set to true
            &RendererStrings {
                renderers: strings
                    .iter()
                    .map(|r| {
                        let mut renderer = r.clone();
                        if *current_renderer.lock().unwrap() == renderer.name {
                            renderer.enabled = true;
                        }
                        renderer
                    })
                    .collect(),
            }
        })
    });

    let cors = warp::cors().allow_any_origin();
    let routes = warp::get().and(renderers).with(cors);

    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;

    Ok(())
}
