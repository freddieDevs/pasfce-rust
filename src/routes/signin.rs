use crate::components::button::{Button, ButtonSize, ButtonVariant};
use leptos::{component, create_server_action, server, view, IntoView, ServerFnError};
use crate::components::input::Input;

#[component]
pub fn SigninPage() -> impl IntoView {
    let _submit = create_server_action::<SignInFn>();
    view! {
      <main>
        <div class="fixed inset-0 z-50 bg-background/80 backdrop-blur-sm">
          <div class="flex items-center justify-center h-screen">
            <div class="fixed z-50 grid w-full max-w-lg mx-auto border rounded-lg p-4 space-y-4 shadow-lg bg-amber-100 text-cyan-900">
              <h1 class="text-center text-2xl font-bold">Sign in</h1>
              <div class="flex justify-between px-2">
                <p class="pr-10 my-auto">Name</p>
                <Input 
                  class_name=String::from("placeholder:text-slate-600")
                  input_type=String::from("text") placeholder=String::from("enter your name")/>
              </div>
              <div class="pt-6 space-x-2 flex items-center justify-center w-ful">
              <Button variant=ButtonVariant::Outline size=ButtonSize::Sm class=String::from("px-2")>Sign in</Button>
              </div>
              
            </div>
          </div>
        </div>
      </main>
    }
    // view! {
    //   <div>
    //     <p>sign in page</p>
    //   </div>
    // }
}

/// all server fns should be async
#[server(SignInFn, "/api/signin")]
pub async fn sign_in_fn() -> Result<(), ServerFnError> {
    todo!()
}
