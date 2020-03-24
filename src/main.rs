extern crate gtk;
extern crate gio;
extern crate qrcode;
// To import all needed traits.
use gtk::prelude::*;
use gio::prelude::*;

use std::env;

fn main() {
    let uiapp = gtk::Application::new(Some("org.simon.qr_code_shower"),
                                      gio::ApplicationFlags::FLAGS_NONE)
                                 .expect("Application::new failed");
    uiapp.connect_activate(|app| {
        // We create the main window.
        let win = gtk::ApplicationWindow::new(app);
        let button = gtk::Button::new_with_label("Generate Qr_Code");
        let data_entry = gtk::Entry::new();
        let output_view = gtk::TextView::new();
        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
        vbox.add(&button);
        vbox.add(&data_entry);
        vbox.add(&output_view);
        button.connect_clicked(move |_|{
            println!("data_entry.get_text() {}, lenght: {}", data_entry.get_text().unwrap().as_str(), data_entry.get_text().unwrap().as_str().len());
            // TODO: add error handling
            let qrcode_generator = qrcode::QrCode::new(String::from(data_entry.get_text().unwrap().as_str()).as_bytes()).unwrap();
            let output_string = qrcode_generator.render::<&str>().quiet_zone(true).light_color("█").dark_color("░").module_dimensions(1, 1).build();
            println!("Code\n{}", output_string);
            output_view.get_buffer().unwrap().set_text(&output_string);

        });

        win.add(&vbox);
        //win.set_default_size(320, 200);
        win.set_title("QRCode_Example");

        // Don't forget to make all widgets visible.
        win.show_all();
    });
    uiapp.run(&env::args().collect::<Vec<_>>());
}
