use leptos::ev::Event;
use leptos::*;
use std::rc::Rc;

// pub struct InputProps {
//     pub class_name: Option<String>,
//     pub input_type: Option<String>,
//     pub on_input: Option<Rc<dyn Fn(Event)>>,
//     pub placeholder: Option<String>,
//     pub disabled: bool,
//     // Add other props as needed
// }

#[component]
pub fn Input(
    #[prop(optional)] on_input: Option<Rc<dyn Fn(Event)>>,
    #[prop(optional)] class_name: Option<String>,
    #[prop(optional)] placeholder: Option<String>,
    #[prop(optional)] disabled: bool,
    #[prop(default= "text".to_string())] input_type: String,
) -> impl IntoView {
    let class = format!(
        "flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-accent focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 {}",
        class_name.unwrap_or_default()
    );

    view! {
        <input
            class=class
            type=input_type
            on:input=move |ev| {
                if let Some(on_input) = on_input.as_ref() {
                    on_input(ev);
                }
            }
            placeholder=placeholder.unwrap_or_else(|| "".into())
            disabled=disabled
        />
    }
}
