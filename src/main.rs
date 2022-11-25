// use gloo_net::http::Request;
use serde::Deserialize;
use yew::prelude::*;

#[derive(Clone, PartialEq, Deserialize)]
struct Video {
    id: usize,
    title: String,
    speaker: String,
    url: String,
}

#[derive(Properties, PartialEq)]
struct VideosListProps {
    videos: Vec<Video>,
    on_click: Callback<Video>,
}

#[derive(Properties, PartialEq)]
struct VideosDetailsProps {
    video: Video,
}

#[function_component(VideoDetails)]
fn video_details(VideosDetailsProps { video }: &VideosDetailsProps) -> Html {
    html! {
        <div>
            <h3>{ video.title.clone() }</h3>
            <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
        </div>
    }
}

#[function_component(VideosList)]
fn videos_list(VideosListProps { videos, on_click }: &VideosListProps) -> Html {
    let on_click = on_click.clone();
    videos.iter().map(|video| {
    let on_video_select = {
        let on_click = on_click.clone();
        let video = video.clone();
        Callback::from(move |_| on_click.emit(video.clone()))
    };

    html! {<p key={video.id} onclick={on_video_select}>{format!("{}: {}", video.speaker, video.title)}</p>}
}).collect()
}

#[function_component(App)]
fn app() -> Html {
    let videos = use_state(|| vec![]);
    {
        let videos = videos.clone();
        use_effect_with_deps(
            move |_| {
                let videos = videos.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    // commented this out because was complaining about unexpected character in json
                    // let fetched_videos: Vec<Video> = Request::get("/tutorial/data.json")
                    //     .send()
                    //     .await
                    //     .unwrap()
                    //     .json()
                    //     .await
                    //     .unwrap();

                    let data = r#"
                    [
                        {
                            "id": 1,
                            "title": "Building and breaking things",
                            "speaker": "John Doe",
                            "url": "https://youtu.be/PsaFVLr8t4E"
                        },
                        {
                            "id": 2,
                            "title": "The development process",
                            "speaker": "Jane Smith",
                            "url": "https://youtu.be/PsaFVLr8t4E"
                        },
                        {
                            "id": 3,
                            "title": "The Web 7.0",
                            "speaker": "Matt Miller",
                            "url": "https://youtu.be/PsaFVLr8t4E"
                        },
                        {
                            "id": 4,
                            "title": "Mouseless development",
                            "speaker": "Tom Jerry",
                            "url": "https://youtu.be/PsaFVLr8t4E"
                        }
                    ]
                    "#;

                    let fetched_videos: Vec<Video> =
                        serde_json::from_str(&&data.to_string()).unwrap();

                    videos.set(fetched_videos);
                });
                || ()
            },
            (),
        );
    }

    let selected_video = use_state(|| None);

    let on_video_select = {
        let selected_video = selected_video.clone();
        Callback::from(move |video: Video| selected_video.set(Some(video)))
    };

    let details = selected_video.as_ref().map(|video| {
        html! {
            <VideoDetails video={video.clone()} />
        }
    });

    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{ "Videos to watch" }</h3>
                 <VideosList videos={(*videos).clone()} on_click={on_video_select.clone()} />
            </div>
            { for details }
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
