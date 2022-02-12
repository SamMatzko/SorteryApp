use gtk::prelude::*;
use gtk;
use sorterylib::prelude::*;
use std::path::Path;
use std::thread;
use std::time::Duration;

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
        .resizable(false)
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
    source_chooser_button.select_filename(Path::new("/home/sam/test/what/"));
    grid.attach(&source_chooser_label, 0, 0, 1, 1);
    grid.attach(&source_chooser_button, 1, 0, 1, 1);

    let target_chooser_label = gtk::Label::new(Some("Target Folder: "));
    let target_chooser_button = gtk::FileChooserButton::new("Choose target...", gtk::FileChooserAction::SelectFolder);
    target_chooser_button.select_filename(Path::new("/home/sam/test/other/"));
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

    // Add the preserve name checkbutton
    let preserve_name_checkbutton = gtk::CheckButton::builder()
        .label("Preserve Name")
        .tooltip_text("Preserve the original file name after the date, separated by \
                    a space. For example, file.txt would be renamed to 2021-02-03 file.txt")
        .active(true)
        .build();
    grid.attach(&preserve_name_checkbutton, 0, 2, 4, 1);

    // The containers for the date type radiobuttons
    let date_type_frame = gtk::Frame::new(Some("Date Type"));
    let date_type_grid = gtk::Grid::builder()
        .column_spacing(5)
        .row_spacing(5)
        .build();
    date_type_frame.add(&date_type_grid);
    grid.attach(&date_type_frame, 0, 3, 4, 1);

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
    grid.attach(&exclude_types_label, 0, 4, 1, 1);
    grid.attach(&exclude_types_entry, 1, 4, 3, 1);

    // The only types entry and its label
    let only_types_label = gtk::Label::new(Some("Only Type(s): "));
    let only_types_entry = gtk::Entry::builder()
        .placeholder_text("Only sort these file extensions...")
        .tooltip_text("File extensions to exclusively sort. Separate with commas, \
            e.g. \"jpg, png, txt\", etc.")
        .build();
    grid.attach(&only_types_label, 0, 5, 1, 1);
    grid.attach(&only_types_entry, 1, 5, 3, 1);

    // The start button
    let start_button = gtk::Button::with_label("Start");
    let start_button_style_context = start_button.style_context();
    start_button_style_context.add_class("suggested-action");
    grid.attach(&start_button, 0, 6, 4, 1);
    start_button.connect_clicked( move |start_button| {

        // The configuration options for the sorter
        let source = File::from(source_chooser_button.filename().expect("Failed to get file name"));
        let target = File::from(target_chooser_button.filename().expect("Failed to get file name"));
        let date_format = date_format_label.text().to_string();
        let date_type = get_date_type(
            created_radiobutton.is_active(),
            accessed_radiobutton.is_active(),
            modified_radiobutton.is_active()
        );
        let preserve_name = preserve_name_checkbutton.is_active();

        // The file types must be correctly split by comma and sorted into a vector
        // for the sorter

        let mut exclude_type: Vec<String> = Vec::new();
        let types = exclude_types_entry.text();
        remove_whitespace(&mut types.to_string());
        for t in types.split(",") {
            let mut s = String::from(t);
            remove_whitespace(&mut s);
            exclude_type.push(String::from(&s));
        }

        let mut only_type: Vec<String> = Vec::new();
        let types = only_types_entry.text();
        remove_whitespace(&mut types.to_string());
        for t in types.split(",") {
            let mut s = String::from(t);
            remove_whitespace(&mut s);
            only_type.push(String::from(&s));
        }

        // Initialize the sorter
        let sorter = Sorter {
            source: source,
            target: target.copy(),
            date_format: date_format,
            date_type: date_type,
            preserve_name: preserve_name,
            exclude_type: exclude_type,
            only_type: only_type
        };

        println!("{:?}", sorter);

        // Show the progress bar
        let progress_bar = gtk::ProgressBar::builder()
            .show_text(true)
            .text(format!("Preparing to sort into \"{}\"", target.file_name()).as_str())
            .build();
        grid.remove(start_button);
        grid.attach(&progress_bar, 0, 6, 4, 1);
        grid.show_all();

        // Run the sorting algorithm with a callback
        sorter.sort_with_callback(true, |d| {
            println!("{}/{}", d.0, d.1);
            progress_bar.set_fraction(d.0 as f64/d.1 as f64);
            progress_bar.set_text(Some(format!(
                "Sorted {} of {} files ({}%)...",
                d.0,
                d.1,
                (d.0 as f64/d.1 as f64) * 100 as f64
            ).as_str()));
            while gtk::events_pending() {
                gtk::main_iteration();
            }
            thread::sleep(Duration::from_millis(50));
        });

    });

    window.show_all();
}

/// Return a string representing created, accessed, or modified based on the bools
/// given as arguments.
fn get_date_type(created: bool, accessed: bool, modified: bool) -> String {
    if created { return String::from("c") }
    else if accessed { return String::from("a") }
    else if modified { return String::from("m") }
    else { return String::from("m") }
}

fn main() {

    // The application
    let app = gtk::Application::builder()
        .application_id("org.sortery.app")
        .build();
    app.connect_activate(create_window);
    app.run();
}