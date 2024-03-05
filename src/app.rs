use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/staxx.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/poll" view=Poll/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Sample poll"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
        <a href="/poll">"Röstning pågår!"</a>
        <h1>"TAGGA DORK"</h1>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}

#[component]
fn Poll() -> impl IntoView {
    return view! {
        <h1>"Hello this is me!"</h1>
        <img src="https://avatars.githubusercontent.com/u/32063345" />

        <form  method="post">
            <label for="inp">"Skicka något till databasen!"</label><br/>
            <input type="text" id="inp" name="inp" /><br/><br/><br/>

            <input type="checkbox" id="box" name="box" value="tagga" />
            <label for="box"> "TAGGA DORK"</label><br/>

            <input type="submit" value="Submit" />
        </form>
    }
}