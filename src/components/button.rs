#![allow(unused)]
use leptos::*;

#[derive(Clone, Copy)]
pub enum ButtonVariant {
    Default,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
}

#[derive(Clone, Copy)]
pub enum ButtonSize {
    Default,
    Sm,
    Lg,
    Icon,
}

// Function to construct class names dynamically
fn button_class(
    variant: ButtonVariant,
    size: ButtonSize,
    additional_classes: Option<&str>,
) -> String {
    let mut classes = vec![
        "inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50",
    ];

    // Add classes based on the variant
    classes.push(match variant {
        ButtonVariant::Default => "bg-primary text-primary-foreground hover:bg-primary/90",
        ButtonVariant::Destructive => {
            "bg-destructive text-destructive-foreground hover:bg-destructive/90"
        }
        ButtonVariant::Outline => {
            "border border-input bg-background hover:bg-accent hover:text-accent-foreground"
        }
        ButtonVariant::Secondary => "bg-secondary text-secondary-foreground hover:bg-secondary/80",
        ButtonVariant::Ghost => "hover:bg-accent hover:text-accent-foreground",
        ButtonVariant::Link => "text-primary underline-offset-4 hover:underline",
    });

    // Add classes based on the size
    classes.push(match size {
        ButtonSize::Default => "h-10 px-4 py-2",
        ButtonSize::Sm => "h-9 rounded-md px-3",
        ButtonSize::Lg => "h-11 rounded-md px-8",
        ButtonSize::Icon => "h-10 w-10",
    });

    // Add any additional classes passed as a prop
    if let Some(extra) = additional_classes {
        classes.push(extra);
    }

    classes.join(" ")
}

#[component]
pub fn Button(
    #[prop(default = ButtonVariant::Default)] variant: ButtonVariant,
    #[prop(default = ButtonSize::Default)] size: ButtonSize,
    #[prop(optional)] on_click: Option<Box<dyn Fn()>>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let class_name = button_class(variant, size, class.as_deref());

    view! {
        <button
            class=class_name
            on:click=move |_| {
                if let Some(callback) = &on_click {
                    callback();
                }
            }
        >
            { children.map(|children| children()) }
        </button>
    }
}
