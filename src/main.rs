// use sorterylib::prelude::*;
use gtk::prelude::*;
use gtk;

/// Remove the whitespace from a [`String`].
fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
}

/// Add all the widgets to the window
fn create_window(app: &gtk::Application) {

    // The application's main window
    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("Sortery")
        .build();
    window.set_position(gtk::WindowPosition::Center);

    // The main grid holding all the widgets, and the main box holding the grid
    let main_vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);
    let main_hbox = gtk::Box::new(gtk::Orientation::Horizontal, 5);
    let grid = gtk::Grid::builder()
        .column_spacing(5)
        .row_spacing(5)
        .build();
    window.add(&main_vbox);
    main_vbox.pack_start(&main_hbox, true, true, 5);
    main_hbox.pack_start(&grid, true, true, 5);

    // Add the file-chooser buttons and their labels
    let source_chooser_label = gtk::Label::new(Some("Source Folder: "));
    let source_chooser_button = gtk::FileChooserButton::new("Choose source...", gtk::FileChooserAction::SelectFolder);
    grid.attach(&source_chooser_label, 0, 0, 1, 1);
    grid.attach(&source_chooser_button, 1, 0, 1, 1);

    let target_chooser_label = gtk::Label::new(Some("Target Folder: "));
    let target_chooser_button = gtk::FileChooserButton::new("Choose target...", gtk::FileChooserAction::SelectFolder);
    grid.attach(&target_chooser_label, 2, 0, 1, 1);
    grid.attach(&target_chooser_button, 3, 0, 1, 1);

    // Add the date_format entry and it's label
    let date_format_label = gtk::Label::new(Some("Date Format: "));
    let date_format_entry = gtk::Entry::builder()
        .placeholder_text("%Y-%m-%d %Hh%Mm%Ss")
        .text("%Y-%m-%d %Hh%Mm%Ss")
        .tooltip_text("The date format used when sorting. Defaults to %Y-%m-%d %Hh%Mm%Ss")
        .build();
    grid.attach(&date_format_label, 0, 1, 1, 1);
    grid.attach(&date_format_entry, 1, 1, 3, 1);

    // The containers for the date type radiobuttons
    let date_type_frame = gtk::Frame::new(Some("Date Type"));
    let date_type_grid = gtk::Grid::builder()
        .column_spacing(5)
        .row_spacing(5)
        .build();
    date_type_frame.add(&date_type_grid);
    grid.attach(&date_type_frame, 0, 2, 4, 1);

    // The date type radiobuttons themselves

    let accessed_radiobutton = gtk::RadioButton::builder()
        .label("Accessed")
        .tooltip_text("Sort by the date the file was accessed.")
        .build();

    let created_radiobutton = gtk::RadioButton::from_widget(&accessed_radiobutton);
    created_radiobutton.set_label("Created");
    created_radiobutton.set_tooltip_text(Some("Sort by the date the file was created."));

    let modified_radiobutton = gtk::RadioButton::from_widget(&accessed_radiobutton);
    modified_radiobutton.set_label("Modified");
    modified_radiobutton.set_tooltip_text(Some("Sort by the date the file was modified."));
    modified_radiobutton.set_active(true);

    date_type_grid.attach(&accessed_radiobutton, 0, 0, 1, 1);
    date_type_grid.attach(&created_radiobutton, 1, 0, 1, 1);
    date_type_grid.attach(&modified_radiobutton, 2, 0, 1, 1);

    // The exclude types entry and its label
    let exclude_types_label = gtk::Label::new(Some("Exclude Type(s): "));
    let exclude_types_entry = gtk::Entry::builder()
        .placeholder_text("File extensions to exclude...")
        .tooltip_text("File extensions to exclude from sorting. Separate with commas, \
            e.g. \"jpg, png, txt\", etc.")
        .build();
    grid.attach(&exclude_types_label, 0, 3, 1, 1);
    grid.attach(&exclude_types_entry, 1, 3, 3, 1);

    // The only types entry and its label
    let only_types_label = gtk::Label::new(Some("Only Type(s): "));
    let only_types_entry = gtk::Entry::builder()
        .placeholder_text("Only sort these file extensions...")
        .tooltip_text("File extensions to exclusively sort. Separate with commas, \
            e.g. \"jpg, png, txt\", etc.")
        .build();
    grid.attach(&only_types_label, 0, 4, 1, 1);
    grid.attach(&only_types_entry, 1, 4, 3, 1);

    // The start button
    let start_button = gtk::Button::with_label("Start");
    let start_button_style_context = start_button.style_context();
    start_button_style_context.add_class("suggested-action");
    grid.attach(&start_button, 0, 5, 4, 1);

    window.show_all();
}

fn main() {

    // The application
    let app = gtk::Application::builder()
        .application_id("org.sortery.app")
        .build();
    app.connect_activate(create_window);
    app.run();
}