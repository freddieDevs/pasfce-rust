use std::rc::Rc;

use crate::components::button::{Button, ButtonSize, ButtonVariant};
use leptos::{component, create_server_action, create_signal, event_target_value, server, view, IntoView, ServerFnError, SignalGet, SignalSet};
use tracing::Event;
use crate::components::input::Input;
use leptos_router::ActionForm;
use serde::{Deserialize, Serialize};
use log::info;

//define the form data Struct
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SignInData {
  pub name: String,
  pub password: String,
}

#[component]
pub fn SigninPage() -> impl IntoView {
  // server action
  let submit = create_server_action::<SignInFn>();
  // create signal to store the data locally
  let (name, set_name) = create_signal(String::new());
  let (password, set_password) = create_signal(String::new());

  let handle_click = move || {
    let data = SignInData {
      name: name.get().clone(),
      password: password.get().clone(),
    };
    // log the formdata to browser console
    info!("Submitting form data: {:?}", data);
    //dispatch the server action with the form data
    submit.dispatch(SignInFn{ data });
  };

  view! {
    <main>
      <div class="fixed inset-0 z-50 bg-background/80 backdrop-blur-sm">
        <div class="flex items-center justify-center h-screen">
          <div class="fixed z-50 grid w-full max-w-lg mx-auto border rounded-lg p-4 space-y-4 shadow-lg bg-amber-100 text-cyan-900">
            <h1 class="text-center text-2xl font-bold">Sign in</h1>
            
            <ActionForm action=handle_click>
            <div class="flex justify-between px-2">
              <p class="pr-10 my-auto">Name</p>
              <Input 
                class_name=String::from("placeholder:text-slate-500")
                input_type=String::from("text") placeholder=String::from("enter your name")
                on_input= move |e| set_name(e.value())/>// update name
                
                </div>
                <div class="flex justify-between px-2">
                <p class="pr-10 my-auto">Password</p>
                <Input 
                class_name=String::from("placeholder:text-slate-500")
                input_type=String::from("password") placeholder=String::from("enter your password")
                on_input= move |e| set_password(e.value())/>
                
                </div>
              <div class="pt-6 space-x-2 flex items-center justify-center w-full">
              <Button variant=ButtonVariant::Outline size=ButtonSize::Lg class=String::from("px-2")
              on_click=Box::new(handle_click)>Sign in</Button>
              </div>
            </ActionForm>
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
pub async fn sign_in_fn(data: SignInData) -> Result<(), ServerFnError> {
  // log form data to the terminal 
  println!("Received form data: {:?}", data);
  Ok(())
}
