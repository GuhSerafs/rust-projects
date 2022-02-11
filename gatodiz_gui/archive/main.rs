use gtk::{prelude::*, Label, Image, Orientation, Box};
use gtk::{Application, ApplicationWindow};

fn main() {
    let app = Application::builder()
        .application_id("org.guhserafs.GatoDiz")
        .build();

    app.connect_activate(|app| {
        // We create the main window.
        let win = ApplicationWindow::builder()
            .application(app)
            .default_width(320)
            .default_height(200)
            .title("Gato Diz!")
            .build();

        let layout_box = Box::new(Orientation::Vertical, 0);
        let label = Label::new(Some("Meow"));
        let img = Image::from_file("./images/gato.jpg");
        layout_box.add(&label);
        layout_box.add(&img);
        // Don't forget to make all widgets visible.
        win.add(&layout_box);
        win.show_all();
    });
    app.run();
}