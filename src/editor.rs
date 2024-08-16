use crate::Terminal;
use termion::event::Key;


const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Position {
    pub x: usize,
    pub y: usize,
}
pub struct Editor{
    should_quit: bool,
    terminal:Terminal,
    cursor_position:Position,
}


impl Editor{

    pub fn run(&mut self){
        loop {
            if let Err(error)= self.refresh_screen(){
                die(&error);
            }
            
            if self.should_quit {
                break;
            }

            if let Err(error) = self.process_keypress(){
                die(&error);
            }

            
        }
    }





    // 默认执行
    pub fn default()->Self{
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
            cursor_position: Position { x: 0, y: 0 },
         }
    }
    // 刷新屏幕
    fn refresh_screen(&self) -> Result<(),std::io::Error>{
        // print!("\x1b[2J");
        // 清空屏幕，并将光标放在左上角
        Terminal::cursor_hide();
        // Terminal::clear_screen();
        // Terminal::cursor_position(0,0);
        Terminal::cursor_position(&Position { x: 0, y: 0 });
        // 打印退出信息
        if self.should_quit{
            Terminal::clear_screen();
            println!("Goodbye.\r");
        }else {
            self.draw_rows();
            // Terminal::cursor_position(0,0);
            Terminal::cursor_position(&self.cursor_position);
            
        }
        Terminal::cursor_show();
        Terminal::flush()
    }
    // 对输入的信息进行处理
    fn process_keypress(&mut self)->Result<(),std::io::Error>{
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            // Key::Ctrl('d') =>panic!("Promgram end"),
            Key::Ctrl('d') => self.should_quit = true,
            Key::Up 
            | Key::Down 
            | Key::Left 
            | Key::Right
            | Key::PageUp 
            | Key::PageDown
            | Key::End
            | Key:: Home => self.move_cursor(pressed_key),
            _ =>(),
        }
        Ok(())

    }
    fn move_cursor(&mut self, key: Key) {
        let Position {mut x, mut y} = self.cursor_position;
        let size =self.terminal.size();
        let height = size.height.saturating_sub(1) as usize;
        let width = size.width.saturating_sub(1) as usize;
        match key {
            Key::Up => y = y.saturating_sub(1),
            // Key::Down => y = y.saturating_add(1),
            Key::Down => {
                if y < height {
                    y = y.saturating_add(1);
                }
            }
            

            Key::Left => x = x.saturating_sub(1),
            // Key::Right => x = x.saturating_add(1),
            Key::Right => {
                if x < width {
                    x = x.saturating_add(1);
                }
            }
            Key::PageUp => y = 0,
            Key::PageDown => y = height,
            Key::Home => x = 0,
            Key::End => x = width,
            _ => (),
        }
        self.cursor_position = Position { x, y }
    }



    fn draw_welcome_message(&self){
        let mut welcome_message = format!("Hecto editor -- version {}",VERSION);
        let width = self.terminal.size().width as usize;
        let len = welcome_message.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{}{}",spaces,welcome_message);
        welcome_message.truncate(width);
        println!("{}\r",welcome_message);
    }



    fn draw_rows(&self){
        // 避免左后一行没有波浪线
        
        let height = self.terminal.size().height;
        for row in 0..height -1{
            // println!("~\r");
            Terminal::clear_current_line();
            if row == height / 3 {
                self.draw_welcome_message();
                // let welcome_message = format!("Hecto editor -- version {}",VERSION);
                // let width = std::cmp::min(self.terminal.size().width as usize, welcome_message.len(),);
                // println!("{}\r",&welcome_message[..width]);
            } else{
                println!("~\r");
            }
        }
    }


    
}

fn die(e: &std::io::Error) {
    // 发生错误时清空屏幕，打印错误
    Terminal::clear_screen();
    panic!("{}",e);
}
