use error_safari::{interface::Safari, run};
use klask::Settings;
use std::borrow::Cow;

fn main() {
    let mut settings = Settings::default();
    settings.custom_font = Some(Cow::Borrowed(include_bytes!(r"../Roboto-Medium.ttf")));

    klask::run_derived::<Safari, _>(settings, run);
}
