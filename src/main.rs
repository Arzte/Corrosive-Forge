/// This is intended to phrase through the `project.json` of all the (pretty much) useless information.
/// Ideally that can be used to make a new CYOA viewer, but at the very least it'll give me some ideas
/// on what to do with the ``project.json`` reduced down without the extra noise.
mod project;

use leptos::*;
use project::Root;
use anyhow::Result;


use crate::project::Required;
use crate::project::Row as RowData;

async fn load_project(_reload_project: bool) -> Result<Vec<project::Row>> {
    let res = reqwasm::http::Request::get("/project.json").send().await.map_err(anyhow::Error::from)?.json::<Root>().await.map_err(anyhow::Error::from)?.rows;
    Ok(res)
}

#[component]
fn Row<'a>(cx: Scope, row_data: &'a RowData) -> impl IntoView {
    let json = serde_json::to_string_pretty::<project::Row>(&row_data).unwrap();
    view! {cx,
        <h3>{&row_data.title}</h3>
        <p>{&row_data.title_text}</p>
        <pre>{json}</pre>
    }
}

// The main actual app
#[component]
fn App(cx: Scope) -> impl IntoView {
    let (project, reload_project) = create_signal(cx, false); 

    let project_root = create_local_resource(cx, project, load_project);
    let values = move || project_root.with(cx, |data| data.unwrap());
            
    let fallback = move |cx, errors: RwSignal<Errors>| {
        let error_list = move || {
            errors.with(|errors| {
                errors
                    .iter()
                    .map(|(_, e)| view! { cx, <li>{e.to_string()}</li>})
                    .collect::<Vec<_>>()
            })
        };

        view! { cx,
            <div class="error">
                <h2>"Error"</h2>
                <ul>{error_list}</ul>
            </div>
        }
    };



    view! { cx,
        <button
            on:click=move |_| {
                reload_project.set(true);
            }
        class:red=move || project() == true
        >
            "Click me to reload project.json (Has been clicked): "
            {project}
        </button>
        <br/>

        <p>"Below should be rows"</p>

        <ErrorBoundary fallback>
            <Transition fallback=move || view! { cx, <div>"Loading project (This may take a few seconds)..."</div>}>
                <For
                    each=values
                    key=|row| row.id
                    view=move |cx, row: RowData| view! { cx, <Row row_data={&row} /> }
                />
            </Transition>
        </ErrorBoundary>
        <p>"Above should be rows"</p>
    }
}

fn main() -> Result<()> {
    // Set this to a empty variable since we don't care about the result.
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    //let v: Root = serde_json::from_str(&contents)?;

    log::info!("it works");
    mount_to_body(|cx| {
        view! { cx, <App /> }
    });
    log::info!("it works the end.");

    Ok(())
}
