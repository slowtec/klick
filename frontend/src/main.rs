use klick_frontend::App;
use leptos::*;

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug); // TODO: use 'Info' in release mode
    console_error_panic_hook::set_once();
    log::info!("Start web application");
    mount_to_body(|| {
        view! { <App /> }
    })
}
