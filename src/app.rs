use leptos::*;
use leptos_meta::*;
use leptos_router::*;
// use leptos_router_macro::path;
use crate::routes::{root::Root, signin::SigninPage};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/static/pasfce.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        
        <Router>  
            // we shall see how to implement this inside root only using the A tag
            // <nav>
            //     <A href="/">"Contacts"</A>
            //     <A href="/about">"About"</A>
            //     <A href="/settings">"Settings"</A>
            // </nav>  
            // here we can define what is going to be seen on all routes
            //TODO: MODALS COME HERE
            <main>
                <Routes>
                    <Route path="" view=  move || view! { <Root/> }/>
                    <Route path= "/signin" view= move || view! {<SigninPage/>} />
                    
                </Routes>
            </main>
            
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    let (value, set_value) = create_signal(0);

    // thanks to https://tailwindcomponents.com/component/blue-buttons-example for the showcase layout
    view! {
        <Title text="Leptos + Tailwindcss"/>
        <main>
            <div class="font-sans bg-gradient-to-tl from-blue-800 to-blue-500 text-gray-100 flex flex-col min-h-screen">
                <div class="flex flex-row-reverse flex-wrap m-auto">
                    <button on:click=move |_| set_value.update(|value| *value += 1) class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-700 border-blue-800 text-green-500">
                        "+ increment"
                    </button>
                    <button class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-800 border-blue-900 text-white">
                        {value}
                    </button>
                    <button on:click=move |_| set_value.update(|value| *value -= 1) class="rounded px-3 py-2 m-1 border-b-4 border-l-2 shadow-lg bg-blue-700 border-blue-800 text-red-800">
                        "- decrement"
                    </button>
                </div>
            </div>
        </main>
    }
}
