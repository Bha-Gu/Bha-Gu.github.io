use leptos::prelude::*;
use leptos_router::components::Outlet;
extern crate alloc;
use alloc::string::String;

const PASSWORD: &str = "letmein"; // Change this to whatever you want

#[component]
pub fn SecretPageLogin() -> impl IntoView {
    let unlocked = RwSignal::new(false);
    let password = RwSignal::new(String::new());
    let error = RwSignal::new(false);

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        if password.get() == PASSWORD {
            unlocked.set(true);
            error.set(false);
        } else {
            error.set(true);
        }
    };

    view! {
        {move || {
            if unlocked.get() {
                view! { <Outlet /> }.into_any()
            } else {
                view! {
                    <div class="secret-login">
                        <div class="card">
                            <h1>"Restricted Access"</h1>
                            <p>"Enter the password to continue."</p>

                            <form on:submit=on_submit>
                                <input
                                    type="password"
                                    placeholder="Password"
                                    prop:value=move || password.get()
                                    on:input=move |ev| password.set(event_target_value(&ev))
                                />

                                <button type="submit">"Unlock"</button>

                                {move || {
                                    error
                                        .get()
                                        .then(|| {
                                            view! { <div class="error">"Incorrect password."</div> }
                                        })
                                }}
                            </form>
                        </div>
                    </div>
                }
                    .into_any()
            }
        }}
    }
}
