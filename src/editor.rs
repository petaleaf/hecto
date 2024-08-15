
use std::io::{self,stdout,Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;


pub struct Editor{
    should_quit: bool,
}
impl Editor{

    pub fn run(&mut self){
        let _stdout = stdout().into_raw_mode().unwrap();
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
        Self {should_quit: false }
    }
    // 刷新屏幕
    fn refresh_screen(&self) -> Result<(),std::io::Error>{
        // print!("\x1b[2J");
        // 清空屏幕，并将光标放在左上角
        print!("{}{}",termion::clear::All, termion::cursor::Goto(1,1));
        // 打印退出信息
        if self.should_quit{
            println!("Goodbye.\r");
        }

        io::stdout().flush()
    }
    // 对输入的信息进行处理
    fn process_keypress(&mut self)->Result<(),std::io::Error>{
        let pressed_key = Self::read_key()?;
        match pressed_key {
            // Key::Ctrl('d') =>panic!("Promgram end"),
            Key::Ctrl('d') => self.should_quit = true,
            _ =>(),
        }
        Ok(())

    }
    // 读取输入
    fn read_key() -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }

}

fn die(e: &std::io::Error) {
    // 发生错误时清空屏幕，打印错误
    print!("{}", termion::clear::All);
    panic!("{}",e);
}
