use fltk::{
    app::{self, App},
    prelude::{DisplayExt, InputExt, WidgetExt},
};

mod terminal;
use self::terminal::Terminal;

pub struct Game {
    terminal: Terminal,
}

impl Game {
    pub fn new() -> Game {
        Game {
            terminal: Terminal::new(),
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
                    self.terminal
                        .print("\n>".to_owned() + &self.terminal.input.value());
                    self.terminal.print("Response".to_string());
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
