use std::fs;
use fltk::{app, prelude::*, *, enums};

const MENU_HEIGHT: i32 = 36;
const WIND_WIDTH: i32 = 920;
const WIND_HEIGHT: i32 = 960;
const SCROLL_WIDTH: i32 = 20;
const FONT_SIZE: i32 = 16;
const OUTPUT_PADDING: i32 = 10;
const PROGRAM_NAME: &str = "File Reader";

#[derive(Clone)]
enum Message {
    Quit,
    Open,
    ScrollEvent,
    ScrollDown,
    ScrollUp,
}

struct FormattedFile {
    name: String,
    line_buffer: Vec<String>,
}

fn main() {
    let mut formatted_file = FormattedFile{
        name: String::from(""),
        line_buffer: vec![String::new(); 1],
    };

    let mut start_index: usize = 0;
    let num_of_lines: usize = ((WIND_HEIGHT - MENU_HEIGHT - OUTPUT_PADDING * 2) / (FONT_SIZE + 3)) as usize;

    let a = app::App::default();
    let (s, r) = app::channel();

    let icon = image::PngImage::load("assets/icon.png").unwrap();

    let mut wind = window::Window::new(
        0,
        0,
        WIND_WIDTH,
        WIND_HEIGHT,
        PROGRAM_NAME,
    );
    wind.set_color(enums::Color::White);
    wind.set_icon(Some(icon));

    let mut menu = menu::SysMenuBar::default().with_size(wind.width(), MENU_HEIGHT);
    menu.set_frame(enums::FrameType::FlatBox);

    menu.add_emit(
        "&File/Open\t",
        enums::Shortcut::Ctrl | 'o',
        menu::MenuFlag::Normal,
        s.clone(),
        Message::Open,
    );

    menu.add_emit(
        "&Scroll/Up\t",
        enums::Shortcut::Ctrl | 'u', //enums::Key::Up,
        menu::MenuFlag::Normal,
        s.clone(),
        Message::ScrollUp,
    );

    menu.add_emit(
        "&Scroll/Down\t",
        enums::Shortcut::Ctrl | 'd', //enums::Key::Down,
        menu::MenuFlag::Normal,
        s.clone(),
        Message::ScrollDown,
    );

    menu.add_emit(
        "&File/Quit\t",
        enums::Shortcut::Ctrl | 'q',
        menu::MenuFlag::Normal,
        s.clone(),
        Message::Quit,
    );

    let mut output = output::MultilineOutput::default();
    output.set_pos(
        OUTPUT_PADDING,
        MENU_HEIGHT + OUTPUT_PADDING,
    );
    output.set_size(
        wind.width() - SCROLL_WIDTH - OUTPUT_PADDING * 2,
        wind.height() - MENU_HEIGHT - OUTPUT_PADDING * 2,
    );
    output.set_frame(enums::FrameType::FlatBox);
    output.set_text_font(enums::Font::Courier);
    output.set_text_size(FONT_SIZE);

    let mut scroll_bar = valuator::Scrollbar::default();
    scroll_bar.set_size(
        SCROLL_WIDTH,
        wind.height() - MENU_HEIGHT
    );
    scroll_bar.set_pos(
        wind.width() - SCROLL_WIDTH,
        MENU_HEIGHT
    );
    scroll_bar.set_minimum(0.0);
    scroll_bar.set_maximum(1.0);
    scroll_bar.set_slider_size(1.0);
    scroll_bar.set_step(1.0, 1);
    scroll_bar.emit(s.clone(), Message::ScrollEvent);

    wind.end();
    wind.show();

    while a.wait() {
        if let Some(msg) = r.recv() {
            match msg {
                Message::Quit => {
                    fltk::app::quit();
                },
                Message::Open => {
                    formatted_file = open_file_dialog();
                    start_index = 0;

                    wind.set_label(&(PROGRAM_NAME.to_string() + "  --  " + &formatted_file.name));

                    set_value_for_output(&mut output, &formatted_file.line_buffer, start_index, num_of_lines);

                    scroll_bar.set_maximum(formatted_file.line_buffer.len() as f64);
                    scroll_bar.set_slider_size((((wind.height() - MENU_HEIGHT - OUTPUT_PADDING * 2) / (FONT_SIZE + 3)) as f32) / (formatted_file.line_buffer.len() as f32));
                },
                Message::ScrollEvent => {
                    set_value_for_output(&mut output, &formatted_file.line_buffer, scroll_bar.value() as usize, num_of_lines);
                },
                Message::ScrollDown => {
                    if formatted_file.line_buffer.len() > 2 && start_index < formatted_file.line_buffer.len() - 2 {
                        start_index = start_index + 1;
                    };
    
                    set_value_for_output(&mut output, &formatted_file.line_buffer, start_index, num_of_lines);
                    scroll_bar.set_value(start_index as f64);
                },
                Message::ScrollUp => {
                    if start_index > 0 {
                        start_index = start_index - 1;
                    };

                    set_value_for_output(&mut output, &formatted_file.line_buffer, start_index, num_of_lines);
                    scroll_bar.set_value(start_index as f64);
                },
            }
        }
    }

    a.run().unwrap();
}

fn set_value_for_output(
    output: &mut output::MultilineOutput,
    value: &Vec<String>,
    start_index: usize,
    num_of_lines: usize,
) {
    let shown_string: String = concat_output_string(
        &value,
        start_index,
        num_of_lines,
    );


    return output.set_value(&shown_string);
}

fn open_file_dialog() -> FormattedFile {
    let mut dialog = dialog::NativeFileChooser::new(dialog::NativeFileChooserType::BrowseFile);
    dialog.show();

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
        return read_file(&file_name);
    }

    return FormattedFile{name: String::from("No file chosen"), line_buffer: vec![String::from(""); 1]};
}

fn concat_output_string(file_data: &Vec<String>, mut first_line_index: usize, mut num_of_lines: usize) -> String {
    let mut res: String = String::new();
    let mut last_line_index: usize = 0;
    if file_data.len() > 0 {
        last_line_index = file_data.len() - 1;
    }

    if first_line_index > last_line_index {
        first_line_index = last_line_index;
    }

    if first_line_index + num_of_lines > last_line_index { //out of bound
        num_of_lines = last_line_index - first_line_index;
    }

    for i in 0..num_of_lines {
        res.push_str(&file_data[first_line_index + i]);
        if i < num_of_lines - 1 {
            res.push('\n');
        }
    }

    return res;
}

fn read_file(file_path: &str) -> FormattedFile {
    let mut file_buff: Vec<u8> = Vec::new();

    match fs::read(file_path) {
        Err(why) => println!("! {:?}", why.kind()),
        Ok(buff) => {
            file_buff = buff;
        }
    };

    return print_file_buffer(file_buff, file_path);
}

fn print_file_buffer(buff: Vec<u8>, name: &str) -> FormattedFile {
    let mut index: usize;
    let mut formatted_file: FormattedFile = FormattedFile { name: String::from(name), line_buffer: Vec::new() };
    

    for j in 0..(buff.len() / 16 + 1) {
        let mut formatted_string: String = String::new();

        formatted_string.push_str(&(j + 1).to_string());
        formatted_string.push_str("\t|  ");

        for i in 0..16 {
            index = j * 16 + i;
            if index < buff.len() {
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
                formatted_string.push(byte_to_char(buff[index]));
            } else {
                formatted_string.push(' ');
            }
        }

        formatted_string.push_str("  |");
        formatted_file.line_buffer.push(formatted_string);
    }

    return formatted_file;
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
    if byte >= 32 && byte < 127 {
        return byte as char;
    } else if byte == 0 {
        return '░';
    } else if byte == 0x7f {
        return '▓';
    } else if byte == 255 {
        return '█';
    } else {
        return '▒';
    }
}