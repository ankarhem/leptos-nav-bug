use leptos::*;
use leptos_router::*;

#[server(GetSomething, "/api")]
pub async fn get_something(id: String) -> Result<String, ServerFnError> {
    Ok(id)
}

#[component]
pub fn Page(cx: Scope) -> impl IntoView {
    let params = use_params_map(cx);

    let id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());
    let data = create_resource(cx, id, move |id| async move { get_something(id).await });
    let link_one = "/hello";

    let rel_one = format!("/{}#h1", "hello");
    let rel_two = format!("/{}#h2", "hello");
    view! { cx,
        <nav class="flex items-center justify-center gap-4">
            <A href="/">"/"</A>
            <A href="/hello">{link_one}</A>
            <A href=rel_one.clone()>{rel_one}</A>
            <A href=rel_two.clone()>{rel_two}</A>
        </nav>
        <Transition fallback=|| ()>
            {move || match data.read(cx) {
                Some(Ok(_)) => {
                    view! { cx,
                        <p>"Navigate within the page by clicking `/hello#h1` you will get _another_ request. Which is unexpected."</p>
                        <p>"Subsequent navigation to other elements within the page, will not trigger."</p>
                        <h1 id="h1">""</h1>
                        <h2 id="h2">""</h2>
                    }.into_view(cx)
                },
                _ => view! { cx,
                    <p>"Loading..."</p>
                }.into_view(cx)
            }}
        </Transition>
    }
}
