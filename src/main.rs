use fltk::{app, prelude::*, *, enums, output::MultilineOutput};

#[derive(Clone)]
enum Message {
    Quit,
    Open,
}

fn main() {
    let a = app::App::default();
    let (s, r) = app::channel();
    let mut wind = window::Window::new(
        256,
        128,
        1024,
        512,
        "File Reader"
    );
    wind.set_color(enums::Color::White);

    let mut menu = menu::SysMenuBar::default().with_size(wind.width(), 35);
    menu.set_frame(enums::FrameType::FlatBox);



    menu.add_emit(
        "&File/Open\t",
        enums::Shortcut::Ctrl | 'o',
        menu::MenuFlag::Normal,
        s.clone(),
        Message::Open,
    );

    menu.add_emit(
        "&File/Quit\t",
        enums::Shortcut::Ctrl | 'q',
        menu::MenuFlag::Normal,
        s,
        Message::Quit,
    );

    let mut output = output::MultilineOutput::default().with_pos(0, 35).with_size(wind.width(), wind.height() - 35);

    wind.end();
    wind.show();

    while a.wait() {
        if let Some(msg) = r.recv() {
            match msg {
                Message::Quit => {
                    println!("quit app");
                    fltk::app::quit();
                },
                Message::Open => {
                    println!("open the file");
                    open_file_dialog(&mut output);
                },
            }
        }
    }

    a.run().unwrap();
}

fn open_file_dialog(output: &mut MultilineOutput) {
    let mut dialog = dialog::NativeFileChooser::new(dialog::NativeFileChooserType::BrowseFile);
    dialog.show();

    println!("{:?}", dialog.filename());

    let file_name: String = dialog.filename().into_os_string().into_string().unwrap();
    
    if file_name.len() > 0 {
        return output.set_value(&file_name);    
    }

    return output.set_value("No file chosen");
}
