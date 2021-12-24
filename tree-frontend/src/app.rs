use reqwasm::http::Request;
use tree_data_schema::{Renderer};
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
                    let renderer_strings: Vec<Renderer> =
                        Request::get("https://tree.dendropho.be//renderers")
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

    let on_video_select = {
        Callback::from(move |renderer: Renderer| {
            wasm_bindgen_futures::spawn_local(async move {
                Request::post("https://tree.dendropho.be//renderer")
                    .body(serde_json::to_string(&renderer.name).unwrap())
                    .header("Content-Type", "application/json")
                    .send()
                    .await
                    .unwrap();
            })
        })
    };

    html! {
        <>
            <div class="header">
            <p class="header_text">{ "Build your own Rusty Tree" }</p>
            </div>
            <div class="banner">
            <div class="banner_icon"></div>
            <p class="banner_text">{ "Rusty Tree" }</p>
            </div>


            <div class="list_renderers">
                {
                    (*renderers).clone().into_iter().map(|r| {
                        let renderer = r.clone();
                        html! {
                            <>
                            <RendererButton renderer={renderer} on_click={on_video_select.clone()} />
                            </>
                        }
                    }).collect::<Html>()
                }
            </div>
            <div class="footer">
           
            <button class="footer_button"> { "View on Github" }</button>

            <p class="footer_attr">{ "@AngelOnFira" }</p>
         
            </div>

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
        <div class="card_holder">
            <div class="card">

                <video class="card_gif" width="320" height="240" autoplay={true} muted={true} loop={true}>
                    <source src={ renderer.link.clone() } type="video/mp4"/>
                </video>

                <div class="card_info">
                    <p class="card_info_name">{ renderer.name.clone() }</p>
                    <p class="card_info_subtitle">{ "Mario runs hapily through the hills"}</p>
                    <p class="card_info_author"> { "@Zesterer"}</p>

                </div>
                <div class="card_tray">
                        <button onclick={on_renderer_click} class="button_launch">{ "Run" }</button>
                    </div>
            </div>
        </div>
    }
}
