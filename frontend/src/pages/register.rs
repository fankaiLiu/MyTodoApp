use leptos::prelude::*;
use leptos::ev;
use leptos_router::hooks::use_navigate;
use crate::store::{use_user_store, use_api_client};
use crate::components::button::{Button, ButtonVariant};
use crate::components::input::{Input, InputType};
use crate::components::form::{Form, FormGroup, FormActions};
use crate::api::auth::{RegisterRequest, register};

#[component]
pub fn RegisterPage() -> impl IntoView {
    let user_store = use_user_store();
    let client = use_api_client();
    let navigate = use_navigate();

    let (username, set_username) = signal(String::new());
    let (email, set_email) = signal(String::new());
    let (phone, set_phone) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let (error, set_error) = signal(Option::<String>::None);
    let (loading, set_loading) = signal(false);

    let on_submit = move |_: ev::SubmitEvent| {
        let username_val = username.get();
        let email_val = email.get();
        let phone_val = phone.get();
        let password_val = password.get();

        if username_val.is_empty() || email_val.is_empty() || password_val.is_empty() {
            set_error.set(Some("Please fill in all required fields".to_string()));
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
            let req = RegisterRequest {
                username: username_val,
                email: email_val,
                phone: phone_val,
                password: password_val,
            };
            match register(&client_clone, &req).await {
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
                <h1 class="auth-title">"Create Account"</h1>
                <p class="auth-subtitle">"Sign up to get started"</p>

                <Form on_submit=Callback::from(on_submit)>
                    <FormGroup label="Username".to_string() required=true>
                        <Input
                            input_type=InputType::Text
                            placeholder="Choose a username".to_string()
                            on_input=Callback::from(move |v: String| set_username.set(v))
                        />
                    </FormGroup>
                    <FormGroup label="Email".to_string() required=true>
                        <Input
                            input_type=InputType::Email
                            placeholder="Enter your email".to_string()
                            on_input=Callback::from(move |v: String| set_email.set(v))
                        />
                    </FormGroup>
                    <FormGroup label="Phone".to_string()>
                        <Input
                            input_type=InputType::Tel
                            placeholder="Enter your phone number".to_string()
                            on_input=Callback::from(move |v: String| set_phone.set(v))
                        />
                    </FormGroup>
                    <FormGroup label="Password".to_string() required=true>
                        <Input
                            input_type=InputType::Password
                            placeholder="Create a password".to_string()
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
                            {move || if loading.get() { "Creating account..." } else { "Sign Up" }}
                        </Button>
                    </FormActions>
                </Form>

                <div class="auth-footer">
                    <span>"Already have an account? "</span>
                    <a href="/login" class="auth-link">"Sign in"</a>
                </div>
            </div>
        </div>
    }
}
