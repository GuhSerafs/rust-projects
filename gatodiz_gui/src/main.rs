use gtk::prelude::*;

fn build_ui(app: &gtk::Application) {
    let glade_src = include_str!("../templates/gatodiz_ui.glade");
    let builder = gtk::Builder::from_string(glade_src);
    let window: gtk::ApplicationWindow = builder.object("appwindow1").unwrap();
    let img_output: gtk::Image = builder.object("img_output").unwrap();
    window.set_application(Some(app));
    setup_callbacks(&builder);
    window.show_all();
    img_output.hide();
}

fn setup_callbacks(builder: &gtk::Builder) {
    // Entradas e saidas
    let msg_input: gtk::Entry = builder.object("msg_input").unwrap();
    let button: gtk::Button = builder.object("generate_btn").unwrap();
    let msg_output: gtk::Label = builder.object("msg_output_label").unwrap();
    let img_output_cloned: gtk::Image = builder.object("img_output").unwrap();
    let is_dead_switch: gtk::Switch = builder.object("is_dead_switch").unwrap();

    // Setup callbacks
    button.connect_clicked(move |_| {
        msg_output.set_text(&format!(
            "{}\n   \\\n     \\",
            msg_input.text().as_str()
        ));

        let is_dead = is_dead_switch.is_active();
        if is_dead {
            img_output_cloned.set_from_file(Some("./images/gato_morto.jpg"));
        } else {
            img_output_cloned.set_from_file(Some("./images/gato.jpg"));
        }
        img_output_cloned.show();
    });
}

fn main() {
    let app = gtk::Application::builder()
        .application_id("org.guhserafs.GatoDiz")
        .build();

    app.connect_activate(|app| {
        // We create the main window.
        build_ui(app);
    });
    app.run();
}