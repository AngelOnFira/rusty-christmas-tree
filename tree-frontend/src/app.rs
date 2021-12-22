use reqwasm::http::Request;
use tree_data_schema::{Renderer, RendererStrings};
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let renderers = use_state(|| vec![]);
    {
        let renderers = renderers.clone();
        use_effect_with_deps(
            move |_| {
                let renderers = renderers.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let renderer_strings: Vec<Renderer> = Request::get("/renderers")
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
    }

    let selected_renderer = use_state(|| None);

    let on_video_select = {
        let selected_renderer = selected_renderer.clone();
        Callback::from(move |renderer: Renderer| selected_renderer.set(Some(renderer)))
    };

    html! {
        <>
            <h1>{ "Tree Patterns" }</h1>
            // <div>
            //     {
            //         renderers.into_iter().map(|r| {
            //             let renderer = r.clone();
            //             html! {
            //                 <>
            //                 <RendererButton renderer={renderer} on_click={on_video_select.clone()} />
            //                 </>
            //             }
            //         }).collect::<Html>()
            //     }

            // </div>
        </>
    }
}

#[derive(Properties, PartialEq)]
struct RendererButtonProps {
    renderer: Renderer,
    on_click: Callback<Renderer>,
}

#[function_component(RendererButton)]
fn renderer_button(RendererButtonProps { renderer, on_click }: &RendererButtonProps) -> Html {
    let on_renderer_click = {
        let on_click = on_click.clone();
        let renderer = renderer.clone();
        Callback::from(move |_| on_click.emit(renderer.clone()))
    };

    html! {
        <button onclick={on_renderer_click}>
            { renderer.name.clone() }
        </button>
    }
}
