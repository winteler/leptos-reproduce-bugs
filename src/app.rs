use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::error_template::{AppError, ErrorTemplate};

#[server]
pub async fn empty_data() -> Result<(), ServerFnError> {
    Ok(())
}

#[server]
pub async fn int_data() -> Result<i32, ServerFnError> {
    Ok(1)
}

#[server]
pub async fn char_data() -> Result<char, ServerFnError> {
    Ok('c')
}

#[server]
pub async fn string_data() -> Result<String, ServerFnError> {
    Ok(String::from("Test"))
}

#[server]
pub async fn bool_data() -> Result<bool, ServerFnError> {
    Ok(true)
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos-reproduce-bugs.css"/>

        // sets the document title
        <Title text="Welcome to [[ProjectName]]"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <div>
                    //<NavigationBar/>
                    <div>
                        <div>
                            <Drawer/>
                        </div>
                        <Routes>
                            <Route path="/" view=Home/>
                            <Route path="/container" view=ContainerHeader>
                                <Route path="/" view=ContainerContent/>
                                <Route path="/object" view=Object/>
                            </Route>
                        </Routes>
                    </div>
                </div>
            </main>
        </Router>
    }
}

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div>
            "Home"
        </div>
    }
}

#[component]
pub fn Drawer() -> impl IntoView {
    let string_data = create_resource(move || (), |_| string_data());

    view! {
        <Suspense>
            <pre>{
                move || {
                    format!("{:?}", string_data.get());
                    view! {
                        <a href="/container">"Container 1"</a>
                    }
                }
            }</pre>
        </Suspense>
    }
}

#[component]
pub fn ContainerHeader() -> impl IntoView {
    let int_data = Resource::once(int_data);
    view! {
        <Suspense>
            <pre>{
                move || {
                    view! {
                        <div>{format!("{:?}", int_data.get())}</div>
                        <Outlet/>
                    }
                }
            }</pre>
        </Suspense>
    }
}

#[component]
pub fn ContainerContent() -> impl IntoView {
    let int_data = Resource::once(char_data);
    view! {
        <Suspense>
            <pre>{
                move || {
                    view! {
                        <div>{format!("{:?}", int_data.get())}</div>
                        <a href="/container/object">"Object 1"</a>
                    }
                }
            }</pre>
        </Suspense>
    }
}

#[component]
pub fn Object() -> impl IntoView {
    let int_data = Resource::once(string_data);
    view! {
        <Suspense>
            <pre>{
                move || {
                    view! {
                        <div>{format!("{:?}", int_data.get())}</div>
                    }
                }
            }</pre>
        </Suspense>
        <Detail/>
    }
}

#[component]
pub fn Detail() -> impl IntoView {
    let char_data = Resource::once(bool_data);
    view! {
        <Suspense>
            <pre>{
                move || {
                    view! {
                        <div>{format!("{:?}", char_data.get())}</div>
                    }
                }
            }</pre>
        </Suspense>
    }
}
