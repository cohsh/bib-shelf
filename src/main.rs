use gtk::prelude::*;

fn main() {
    let application =
        gtk::Application::new(Some("com.github.cohsh.pdf-bib"), Default::default());
    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    window.set_title(Some("PDF-bib"));
    window.set_default_size(1200, 600);
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 10);
    window.set_child(Some(&vbox));
    window.show();
}