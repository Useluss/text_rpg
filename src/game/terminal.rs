use fltk::{
    prelude::*, window::Window, 
    enums::{Color, FrameType, Font, CallbackTrigger}, 
    input::Input, output::Output, 
    group::Row, 
    text::{TextDisplay, TextBuffer}, 
};

pub struct Terminal {
    window: Window,
    pub input: Input,
    input_caret: Output,
    header: Row,
    header_info: Vec<Output>,
    output_window: TextDisplay,
    pub output_buffer: TextBuffer,
}

const HEIGHT: i32 = 450;
const WIDTH: i32 = 900;

impl Terminal {
    pub fn new() -> Terminal {
        let font = Font::load_font("fonts/console.ttf").unwrap();
        Font::set_font(Font::Helvetica, &font);
        Terminal { 
            window: Window::new(0, 0, WIDTH, HEIGHT, "My Window!"), input: Input::new(10, 425, WIDTH, 25, ""), 
            input_caret: Output::new(0, 425, 10, 25, ""), header: Row::new(0, 0, WIDTH, 25, ""), 
            header_info: vec![Output::default_fill(), Output::default_fill(), Output::default_fill()],
            output_window: TextDisplay::new(0, 30, WIDTH, 400, ""), output_buffer: TextBuffer::default(),
        }
    }

    pub fn setup_window(&mut self) {
        self.setup_output();
        self.setup_header();
        self.setup_input();
        
        self.window.set_color(Color::Black);
        self.window.end();
        self.window.show();
    }

    fn setup_input(&mut self) {
        self.input.set_frame(FrameType::FlatBox);
        self.input.set_color(Color::Black);
        self.input.set_text_color(Color::White);
        self.input.set_text_size(20);
        self.input.set_text_font(Font::Helvetica);
        self.input.set_selection_color(Color::White);
        self.input.set_trigger(CallbackTrigger::EnterKey);
        self.setup_input_caret();
    }

    fn setup_input_caret(&mut self) {
        self.input_caret.set_value(">");
        self.input_caret.set_frame(FrameType::FlatBox);
        self.input_caret.set_color(Color::Black);
        self.input_caret.set_text_color(Color::White);
        self.input_caret.set_text_size(20);
        self.input_caret.set_text_font(Font::Helvetica);
        self.input_caret.set_selection_color(Color::White);
    }

    fn setup_header(&mut self) {
        self.header.set_frame(FrameType::FlatBox);
        self.header.set_color(Color::gray_scale(155));
        self.header.set_selection_color(Color::White);
        self.header_info[0].set_value("Location");
        self.header_info[1].set_value("Moves: 0");
        self.header_info[2].set_value("Health: 100");

        for i in &mut self.header_info {
            self.header.add(i);
            i.set_frame(FrameType::NoBox);
            i.set_text_size(20);
            i.set_selection_color(Color::White);
        }
    }

    fn setup_output(&mut self) {
        self.header.remove(&self.output_window);
        self.window.add(&self.output_window);
        self.output_window.set_frame(FrameType::FlatBox);
        self.output_window.set_color(Color::Black);
        self.output_window.set_buffer(self.output_buffer.clone());
        self.output_window.set_text_font(Font::Helvetica);
        self.output_window.set_text_color(Color::White);
        self.output_window.set_text_size(20);
        self.output_window.set_selection_color(Color::White);
        self.output_window.scroll(1, 9);
    }

    pub fn print(&mut self, text: String) {
        self.output_buffer.append(&text);
        self.output_buffer.append("\n");
    }
}
