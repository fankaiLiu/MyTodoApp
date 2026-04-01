use leptos::prelude::*;
use leptos::ev;
use leptos_router::hooks::use_navigate;
use crate::store::{use_user_store, use_api_client};
use crate::components::button::{Button, ButtonVariant};
use crate::components::input::{Input, InputType};
use crate::components::form::{Form, FormGroup, FormActions};
use crate::api::auth::{LoginRequest, login};

#[component]
pub fn LoginPage() -> impl IntoView {
    let user_store = use_user_store();
    let client = use_api_client();
    let navigate = use_navigate();

    let (email, set_email) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let (error, set_error) = signal(Option::<String>::None);
    let (loading, set_loading) = signal(false);

    let on_submit = move |_: ev::SubmitEvent| {
        let email_val = email.get();
        let password_val = password.get();

        if email_val.is_empty() || password_val.is_empty() {
            set_error.set(Some("Please fill in all fields".to_string()));
            return;
        }

        set_loading.set(true);
        set_error.set(None);

        let client_clone = client.clone();
        let user_store_clone = user_store.clone();
        let navigate_clone = navigate.clone();
        let set_loading_clone = set_loading;
        let set_error_clone = set_error;

        wasm_bindgen_futures::spawn_local(async move {
            let req = LoginRequest {
                email: email_val,
                password: password_val,
            };
            match login(&client_clone, &req).await {
                Ok(resp) => {
                    user_store_clone.login(resp.access_token, resp.user);
                    navigate_clone("/dashboard", Default::default());
                }
                Err(e) => {
                    set_error_clone.set(Some(e.message));
                    set_loading_clone.set(false);
                }
            }
        });
    };

    view! {
        <div class="auth-page">
            <div class="auth-container">
                <h1 class="auth-title">"Welcome Back"</h1>
                <p class="auth-subtitle">"Sign in to your account"</p>

                <Form on_submit=Callback::from(on_submit)>
                    <FormGroup label="Email".to_string() required=true>
                        <Input
                            input_type=InputType::Email
                            placeholder="Enter your email".to_string()
                            on_input=Callback::from(move |v: String| set_email.set(v))
                        />
                    </FormGroup>
                    <FormGroup label="Password".to_string() required=true>
                        <Input
                            input_type=InputType::Password
                            placeholder="Enter your password".to_string()
                            on_input=Callback::from(move |v: String| set_password.set(v))
                        />
                    </FormGroup>

                    {move || error.get().map(|msg| {
                        view! { <div class="auth-error">{msg}</div> }.into_any()
                    }).unwrap_or_else(|| ().into_any())}

                    <FormActions>
                        <Button
                            variant=ButtonVariant::Primary
                            full_width=true
                            disabled=loading.get()
                        >
                            {move || if loading.get() { "Signing in..." } else { "Sign In" }}
                        </Button>
                    </FormActions>
                </Form>

                <div class="auth-footer">
                    <span>"Don't have an account? "</span>
                    <a href="/register" class="auth-link">"Sign up"</a>
                </div>
            </div>
        </div>
    }
}
