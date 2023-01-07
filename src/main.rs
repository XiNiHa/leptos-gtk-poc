use gtk::gdk::Display;
use gtk::prelude::*;
use gtk::{
    Align, Application, ApplicationWindow, Box, Button, CssProvider, Orientation, StyleContext,
};
use leptos::*;

const APP_ID: &str = "dev.xiniha.HelloGtk";

fn main() {
    _ = create_scope(RuntimeId::default(), |cx| {
        let app = Application::builder().application_id(APP_ID).build();

        app.connect_activate(move |app| build_ui(cx, app));

        app.run();
    })
}

#[derive(Clone)]
enum Color {
    Red,
    Blue,
}

impl Color {
    fn flip(&self) -> Self {
        match self {
            Color::Red => Color::Blue,
            Color::Blue => Color::Red,
        }
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Color::Red => "red",
            Color::Blue => "blue",
        })
    }
}

fn build_style<F>(cx: Scope, id: String, selector: Option<&'static str>, f: F)
where
    F: Fn() -> String + 'static,
{
    let (provider, _) = create_signal(cx, CssProvider::new());

    create_effect(cx, move |_| {
        provider
            .get()
            .load_from_data(format!(r#".{}{} {{ {} }}"#, id, selector.unwrap_or(""), f()).as_bytes())
    });

    StyleContext::add_provider_for_display(
        &Display::default().unwrap(),
        &provider.get(),
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_button(cx: Scope) -> Button {
    let (id, _) = create_signal(cx, cuid::cuid().unwrap());

    let (default_color, set_default_color) = create_signal(cx, Color::Red);
    let (hover_color, set_hover_color) = create_signal(cx, Color::Blue);

    build_style(cx, id.get(), None, move || {
        format!("color: {}; transition: 0.5s ease;", default_color.get())
    });
    build_style(cx, id.get(), Some(":hover"), move || {
        format!("color: {};", hover_color.get())
    });

    let el = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    el.add_css_class(&id.get());

    el.connect_clicked(move |_| {
        set_default_color.set(default_color.get().flip());
        set_hover_color.set(hover_color.get().flip());
    });

    el
}

fn build_ui(cx: Scope, app: &Application) {
    let box_ = Box::builder()
        .orientation(Orientation::Vertical)
        .valign(Align::Center)
        .halign(Align::Fill)
        .build();

    box_.append(&build_button(cx));
    box_.append(&build_button(cx));
    box_.append(&build_button(cx));

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Hello, GTK!")
        .child(&box_)
        .build();

    window.present();
}
