use fltk::{
    app::{self, App},
    prelude::{DisplayExt, InputExt, WidgetExt},
};

mod command_processor;
mod terminal;
use self::terminal::Terminal;

pub struct Game<'a> {
    terminal: Terminal,
    commands: Vec<&'a str>,
}

impl Game<'_> {
    pub fn new() -> Game<'static> {
        Game {
            terminal: Terminal::new(),
            commands: vec!["help", "go [direction]"],
        }
    }

    pub fn run(mut self) {
        let app = App::default();
        self.terminal.setup_window();

        self.on_enter(app);

        app.run().unwrap();
    }

    fn on_enter(mut self, app: App) {
        let (s, r) = app::channel::<()>();

        self.terminal.input.emit(s, ());
        while app.wait() {
            match r.recv() {
                Some(..) => {
                    let input = &self.terminal.input.value();
                    self.terminal
                        .print("\n>".to_owned() + &self.terminal.input.value());
                    let response = command_processor::process_command(&input.to_string());
                    self.terminal.print(response);
                    self.terminal.input.set_value("");
                    // Whacky number works somehow don't change!!!!!!!!!!!!!
                    self.terminal.output_window.scroll(100000, 0);
                }
                None => (),
            }
            std::thread::sleep(std::time::Duration::from_millis(16));
        }
    }
}
