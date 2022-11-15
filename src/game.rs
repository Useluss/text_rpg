use fltk::{app::{App, self}, prelude::{WidgetExt, InputExt}};

mod terminal;
use self::terminal::Terminal;

pub struct Game {
    terminal: Terminal,
} 

impl Game {
    pub fn new() -> Game {
        Game { terminal: Terminal::new() }
    }

    pub fn run(mut self) {
        let app = App::default();
        self.terminal.setup_window();

        let (s, r) = app::channel::<String>();
        
        self.terminal.input.emit(s, String::new()); 
        while app.wait() {
            match r.recv() {
                Some(..) => {
                    self.terminal.print(">".to_owned() + &self.terminal.input.value());
                    self.terminal.print("Response".to_string());
                    self.terminal.input.set_value("");
                }
                None => (),
            }
            std::thread::sleep(std::time::Duration::from_millis(16));
        }

        app.run().unwrap();
    }
}
