//! This crate allows you to quickly program a shell interface with gtk4

use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button, Label, Box, Orientation};

#[allow(dead_code)] //Since i purposely dont use all functions in the operations module
mod operations;

const APP_ID: &str = "org.lucajoshuaweiss.ShellInterface";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    
    //UI elements
    let uptime_status = Label::builder()
        .label("Uptime")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let uptime = Button::builder()
        .label("Uptime")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    let reboot = Button::builder()
        .label("Reboot")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    let poweroff = Button::builder()
        .label("Power Off")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

    let content = Box::new(Orientation::Vertical, 0);
    content.append(&uptime_status);
    content.append(&uptime);
    content.append(&reboot);
    content.append(&poweroff);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Shell Interface")
        .resizable(false)
        .child(&content)
        .build();


    //Connection of UI elements with functions
    uptime.connect_clicked(move |_| operations::operation_with_status(&uptime_status, "bash", "-c", "uptime"));
    reboot.connect_clicked(move |_| operations::operation("bash", "-c", "reboot"));
    poweroff.connect_clicked(move |_| operations::operation("bash", "-c", "poweroff"));

    window.present();
}
