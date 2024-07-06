use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, TreeView, ListStore, TreeViewColumn, CellRendererText, Orientation, TreeModel};
use std::path::{Path, PathBuf};
use std::fs;

fn main() {
    let application = Application::new(Some("de.beklauter.explorer"), Default::default());

    application.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Rust File Explorer")
            .default_width(800)
            .default_height(600)
            .build();

        let vbox = gtk::Box::new(Orientation::Vertical, 5);
        let tree_view = TreeView::new();
        let list_store = ListStore::new(&[String::static_type(), String::static_type()]);

        setup_tree_view(&tree_view);
        tree_view.set_model(Some(&list_store));
        populate_with_drives(&list_store);

        vbox.append(&tree_view);
        window.set_child(Some(&vbox));
        window.show();
    });

    application.run();
}

fn setup_tree_view(tree_view: &TreeView) {
    for i in 0..2 {
        let column = TreeViewColumn::new();
        let cell = CellRendererText::new();

        column.pack_start(&cell, true);
        column.add_attribute(&cell, "text", i as i32);
        tree_view.append_column(&column);
    }
}

fn populate_with_drives(list_store: &ListStore) {
    list_store.clear();

    if let Ok(drives) = fs::read_dir(Path::new("/")) {
        for drive in drives.filter_map(Result::ok) {
            let drive_name = drive.path().display().to_string();
            list_store.insert_with_values(None, &[(0, &drive_name), (1, &drive_name)]);
        }
    }
}

fn populate_with_directory_contents(list_store: &ListStore, directory: &Path) {
    list_store.clear();

    if let Ok(entries) = fs::read_dir(directory) {
        for entry in entries.filter_map(Result::ok) {
            let file_name = entry.file_name().to_string_lossy().into_owned();
            let file_type = if entry.file_type().map(|f| f.is_dir()).unwrap_or(false) {
                "Directory"
            } else {
                "File"
            };

            list_store.insert_with_values(None, &[(0, &file_name), (1, &file_type)]);
        }
    }
}
