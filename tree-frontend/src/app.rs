use reqwasm::http::Request;
use tree_writer::{Renderer, RendererStrings};
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let renderers = use_state(|| vec![]);
    {
        let renderers = renderers.clone();
        use_effect_with_deps(
            move |_| {
                let renderers = renderers.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let renderer_strings: Vec<Renderer> =
                        Request::get("/renderers")
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();
                    renderers.set(renderer_strings);
                });
                || ()
            },
            (),
        );

        let renderers_iter = renderers.as_ref();
    }

    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <RendererButton renderer={videos} on_click={on_video_select.clone()} />
            </div>
            { for renderers_iter }
        </>
    }
}

#[derive(Properties, PartialEq)]
struct RendererButtonProps {
    renderer: Renderer,
    on_click: Callback<Video>,
}

#[function_component(RendererButton)]
fn renderer_button(props: &RendererButtonProps) -> Html {
    let on_click = use_state(|| false);
    let on_click = on_click.clone();
    let on_click = move |_| {
        on_click.set(!on_click.get());
    };

    html! {
        <button onclick=on_click>
            { props.videos.len() }
        </button>
    }
}
