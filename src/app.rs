mod page;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use page::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! { cx,
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="/" view=HomePage/>
                    <Route path="/:id" view=Page/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    view! { cx,
        <A href="/hello">"/hello"</A>
        <p>"Open network tab and go to `/hello`, you will see a network request to the api as expected."</p>
    }
}
