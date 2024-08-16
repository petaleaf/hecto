use crate::Terminal;
use termion::event::Key;



pub struct Editor{
    should_quit: bool,
    terminal:Terminal,
}
impl Editor{

    pub fn run(&mut self){
        loop {
            if let Err(error)= self.refresh_screen(){
                die(&error);
            }

            if let Err(error) = self.process_keypress(){
                die(&error);
            }

            if self.should_quit {
                break;
            }
            
        }
    }





    // 默认执行
    pub fn default()->Self{
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
         }
    }
    // 刷新屏幕
    fn refresh_screen(&self) -> Result<(),std::io::Error>{
        // print!("\x1b[2J");
        // 清空屏幕，并将光标放在左上角
        Terminal::clear_screen();
        Terminal::cursor_position(0,0);
        // 打印退出信息
        if self.should_quit{
            println!("Goodbye.\r");
        }else {
            self.draw_rows();
            Terminal::cursor_position(0,0);
            
        }

        Terminal::flush()
    }
    // 对输入的信息进行处理
    fn process_keypress(&mut self)->Result<(),std::io::Error>{
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            // Key::Ctrl('d') =>panic!("Promgram end"),
            Key::Ctrl('d') => self.should_quit = true,
            _ =>(),
        }
        Ok(())

    }
    fn draw_rows(&self){
        for _ in 0..self.terminal.size().height{
            println!("~\r");
        }
    }


    
}

fn die(e: &std::io::Error) {
    // 发生错误时清空屏幕，打印错误
    Terminal::clear_screen();
    panic!("{}",e);
}
