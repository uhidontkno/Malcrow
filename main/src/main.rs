#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// ^^^ Do not show console window in release mode

use gtk::{prelude::*, Button};
use gtk::{Application, ApplicationWindow};

fn main() {
    let app = Application::builder()
        .application_id("com.rare1k.malcrow")
        .build();

    app.connect_activate(|app| {
        // We create the main window.
        let win = ApplicationWindow::builder()
            .application(app)
            .default_width(320)
            .default_height(200)
            .title("Hello, World!")
            .build();

        let button = Button::with_label("Click me!");
        button.connect_clicked(|_| {
            eprintln!("Clicked!");
        });
        win.add(&button);

        // Don't forget to make all widgets visible.
        win.show_all();
    });

    app.run();
}