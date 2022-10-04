use std::fs;
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

    let mut output = output::MultilineOutput::default().with_pos(10, 45).with_size(wind.width() - 20, wind.height() - 55);
    output.set_frame(enums::FrameType::FlatBox);
    output.set_text_font(enums::Font::Courier);

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

    let mut string_with_newlines: String = String::new();

    for c in file_name.chars() {
        if c == '/' {
            string_with_newlines.push('\n');
        } else {
            string_with_newlines.push(c);
        }
    }
    
    if file_name.len() > 0 {
        let file: String = read_file(&file_name);

        return output.set_value(&file);
    }

    return output.set_value("No file chosen");
}

fn read_file(file_path: &str) -> String {
    let mut file_buff: Vec<u8> = Vec::new();

    match fs::read(file_path) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(buff) => {
            file_buff = buff;
        }
    };

    return print_file_buffer(file_buff);
}

fn print_file_buffer(buff: Vec<u8>) -> String {
    let mut index: usize;
    let mut formatted_string: String = String::new();

    for j in 0..(buff.len() / 16 + 1) {
        /* if (j % 16) == 0 {
            println!("Block #{}", j / 16 + 1);
        } */

        // print!("{}\t|  ", j + 1);
        formatted_string.push_str(&(j + 1).to_string());
        formatted_string.push_str("\t|  ");

        for i in 0..16 {
            index = j * 16 + i;
            if index < buff.len() {
                // print!("{} ", byte_to_hex_string(buff[index]));
                formatted_string.push_str(&byte_to_hex_string(buff[index]));
                formatted_string.push(' ');
            } else {
                formatted_string.push_str("   ");
            }
            if (i + 1) % 4 == 0 {
                formatted_string.push(' ');
            }
            
            if i == 7 {
                formatted_string.push(' ');
            }        
        }
        formatted_string.push_str("|  ");
        for i in 0..16 {
            index = j * 16 + i;
            if index < buff.len() {
                // print!("{}", byte_to_char(buff[index]));
                formatted_string.push(byte_to_char(buff[index]));
            } else {
                formatted_string.push(' ');
            }
        }
        // println!("  |");
        formatted_string.push_str("  |\n");
        
        /* if (j + 1) % 16 == 0 {
            println!("Enter to continue, any key to abort:");
            answer = read_input("qwer");
            if answer != "" {
                println!("answer: {}", answer);
                break;
            }
        } */
    }

    return formatted_string;
}

fn byte_to_hex_string(byte: u8) -> String {
    let high_nibble: u8 = &byte >> 4;
    let low_nibble: u8 = &byte & 0x0f;

    let mut res: String = String::from(nybble_to_char(high_nibble));

    res.push(nybble_to_char(low_nibble));

    return res;
}

fn nybble_to_char(nybble: u8) -> char {
    if nybble < 16 {
        if nybble < 10 {
            return (nybble | 0x30) as char;
        } else {
            return (nybble + 55) as char;
        }
    } else {
        panic!("Nybble should be number less than 16!");
    }
}

fn byte_to_char(byte: u8) -> char {
    if byte >= 32 && byte < 128 {
        return byte as char;
    } else if byte == 0 {
        return '░';
    } else if byte == 255 {
        return '█';
    } else {
        return 26 as char;
    }
}

/* fn read_input<T: std::str::FromStr>(error_message: &str) -> T {
    // generic function that reads input and checks for type (number or text)
    // the error handling should be here
    let mut input: String = String::new();
    let result: T;
    let error: &str;

    if error_message.trim() == "" {
        error = "use correct value type, e.g. number";
    } else {
        error = error_message;
    }

    loop {
        io::stdin()
        .read_line(&mut input)
        .expect("failed to read the line");

        input = input.trim().to_string();

        match input.parse::<T>() {
            Ok(parsed_value) => {
                result = parsed_value;
                break;
            },
            Err(_) => {
                println!("{}", error);
                input = String::new();
                continue;
            },
        };
    }

    return result;
} */