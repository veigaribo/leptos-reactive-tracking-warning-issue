use codee::string::FromToStringCodec;
use leptos::prelude::*;
use leptos_use::use_cookie;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options />
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // let (x, set_x) = signal("Test");
    let (x, set_x) = use_cookie::<String, FromToStringCodec>("test");

    view! { <button>{x.get()}</button> }
}
