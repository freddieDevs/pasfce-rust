use leptos::{component, create_server_action, server, view, IntoView, ServerFnError};
use leptos_router::ActionForm;

#[component]
pub fn SigninPage() -> impl IntoView {
  let submit = create_server_action::<SignInFn>();
  view! {
    <main>
      <div class="fixed inset-0 z-50 bg-background/80 backdrop-blur-sm">
        <div class="flex items-center justify-center h-screen">
          <div class="fixed z-50 grid w-full max-w-lg mx-auto border rounded-lg p-4 shadow-lg bg-amber-100 text-cyan-900">
          <ActionForm action=submit>
            <h1 class="text-center text-2xl font-bold">Sign in</h1>
            <div>
              <div class="space-y-4 py-2 pb-4">
                <p>sign in content</p>
              </div>
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
pub async fn sign_in_fn() -> Result<(), ServerFnError> {
  todo!()
}