use gtk::{Application};
//use gtk4::glib as glib;
//use gtk4::cairo as cairo;
use gtk::gdk as gdk;
use gtk::prelude::*;
use gdk::Display;

mod circular_progess_bar;

fn main() {
    let app = gtk::Application::builder()
        .application_id("com.github.gtk4-rs-test.cosmo")
        .build();
    app.connect_startup(|app| {
        // The CSS "magic" happens here.
        let provider = gtk::CssProvider::new();
        provider.load_from_data(include_str!("style.css"));
        // We give the CssProvided to the default screen so the CSS rules we added
        // can be applied to our window.
        gtk::style_context_add_provider_for_display(
            &Display::default().expect("Could not connect to a display."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        app.connect_activate(build_ui);
    });
    app.connect_activate(build_ui);
    app.run();
}


fn build_ui(app: &gtk::Application) {
    let widget = circular_progess_bar::CircularProgressBar::new();
    let window = gtk::ApplicationWindow::builder()
        .title("Cosmo Test GTK4 RS")
        .application(app)
        .child(&widget)
        .build();
    window.show()
}
