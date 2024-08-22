use crate::Document;
use crate::Terminal;
use crate::Row;
use std::env;
use termion::event::Key;
use termion::color;

const STATUS_FG_COLOR: color::Rgb = color::Rgb(63,63,63);
const STATUS_BG_COLOR: color::Rgb = color::Rgb(239,239,239);
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}
pub struct Editor{
    should_quit: bool,
    terminal:Terminal,
    cursor_position:Position,
    offset: Position,
    document: Document,
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
        let args: Vec<String> = env::args().collect();
        let document = if args.len() >1{
            let file_name = &args[1];
            Document::open(&file_name).unwrap_or_default()
        }else {
            Document::default()
        };



        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
            // cursor_position: Position { x: 0, y: 0 },
            cursor_position: Position::default(),
            document: document,
            // document: Document::open(),
            offset: Position::default(),
         }
    }
    // 刷新屏幕
    fn refresh_screen(&self) -> Result<(),std::io::Error>{
        // print!("\x1b[2J");
        // 清空屏幕，并将光标放在左上角
        Terminal::cursor_hide();
        // Terminal::clear_screen();
        // Terminal::cursor_position(0,0);
        // Terminal::cursor_position(&Position { x: 0, y: 0 });
        Terminal::cursor_position(&Position::default());

        
        // 打印退出信息
        if self.should_quit{
            Terminal::clear_screen();
            println!("Goodbye.\r");
        }else {
            self.draw_rows();
            // Terminal::cursor_position(0,0);
            // Terminal::cursor_position(&self.cursor_position);
            self.draw_status_bar();
            self.draw_message_bar();
            Terminal::cursor_position(&Position{
                x: self.cursor_position.x.saturating_sub(self.offset.x),
                y: self.cursor_position.y.saturating_sub(self.offset.y),
            })
            
        }
        // Ok(())
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
        self.scroll();
        Ok(())

    }
    // 我们的策略是检查光标是否移出了可视窗口，如果移出去了，则调整 offset 让光标刚好在可视窗口内
    fn scroll(&mut self){
        let Position {x,y} = self.cursor_position;
        let width = self.terminal.size().width as usize;
        let height = self.terminal.size().height as usize;
        let mut offset = &mut self.offset;
        if y < offset.y {
            offset.y = y;
        }else if y > offset.y.saturating_add(height) {
            offset.y = y.saturating_sub(height).saturating_add(1);
        }
        if x < offset.x {
            offset.x = x;
        }else if x > offset.x.saturating_add(width){
            offset.x = x.saturating_sub(width).saturating_add(1);
        }
    }
    fn move_cursor(&mut self, key: Key) {
        let terminal_height = self.terminal.size().height as usize;
        let Position {mut x, mut y} = self.cursor_position;
        // let size =self.terminal.size();
        // let height = size.height.saturating_sub(1) as usize;
        let height = self.document.len();
        // let width = size.width.saturating_sub(1) as usize;
        // 对水平方向的移动进行限制
        let mut width = if let Some(row) = self.document.row(y){
            row.len()

        }else{
            0
        };
        match key {
            Key::Up => y = y.saturating_sub(1),
            // Key::Down => y = y.saturating_add(1),
            Key::Down => {
                if y < height {
                    y = y.saturating_add(1);
                }
            }
            

            // Key::Left => x = x.saturating_sub(1),
            // 行首再向前移动则移动至上一行的行尾
            Key::Left => {
                if x >0 {
                    x-=1;
                }else if y >0 {
                    y -=1;
                    if let Some(row) = self.document.row(y){
                        x = row.len();

                }else{
                    x = 0;
                }
                }
            }
            // Key::Right => x = x.saturating_add(1),
            // 光标在行尾的时候再向后移动，则移动到下一行的行首
            Key::Right => {
                if x < width {
                    // x = x.saturating_add(1)；
                    x +=1;
                }else if y < height-1 {
                    y +=1;
                    x = 0;
                }
            }
            
            // Key::PageUp => y = 0,
            // Key::PageDown => y = height,
            Key::PageUp => {
                y = if y> terminal_height {
                    y - terminal_height
                }else {
                    0
                }
            }
            Key::PageDown => {
                y = if y.saturating_add(terminal_height) < height {
                    y + terminal_height as usize
                }else{
                    height
                }
            }
            Key::Home => x = 0,
            Key::End => x = width,
            _ => (),
        }
        // 按键过程中换了行，要重新限定x的值，避免超过行的宽度
        width = if let Some(row) = self.document.row(y) {
            row.len()
        }else {
            0
        };
        if x > width{
            x = width;
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

    pub fn draw_row(&self, row: &Row){
        // let start = 0;
        // let end = self.terminal.size().width as usize;
        // 展示文件每行的正确范围
        let width = self.terminal.size().width as usize;
        let start = self.offset.x;
        let end = self.offset.x + width;
        let row = row.render(start,end);
        println!("{}\r",row)
    }


    fn draw_rows(&self){
        // 避免左后一行没有波浪线
        
        let height = self.terminal.size().height;
        // for row in 0..height -1{
        // for terminal_row in 0..height-1{
        for terminal_row in 0..height{
            // println!("~\r");
            Terminal::clear_current_line();
            // if row == height / 3 {
            // if let Some(row) = self.document.row(terminal_row as usize) {
            if let Some(row) = self.document.row(terminal_row as usize + self.offset.y){
                self.draw_row(row);
                // let welcome_message = format!("Hecto editor -- version {}",VERSION);
                // let width = std::cmp::min(self.terminal.size().width as usize, welcome_message.len(),);
                // println!("{}\r",&welcome_message[..width]);
            // } else if terminal_row == height/3 {
            }else if self.document.is_empty() && terminal_row == height/3 {
                self.draw_welcome_message();

            }else{
                println!("~\r");
            }
        }
    }
    fn draw_status_bar(&self){
        // let spaces = " ".repeat(self.terminal.size().width as usize);
        let mut status;
        let width: usize = self.terminal.size().width as usize;
        let mut file_name: String = "[No Name]".to_string();
        if let Some(name) = &self.document.file_name {
            file_name = name.clone();
            file_name.truncate(20);
        }
        // 拼接要在状态栏显示的信息
        status = format!("{} ❤❤❤ {} lines", file_name,self.document.len());
        // if width > status.len() {
            // status.push_str(&" ".repeat(width-status.len()));


        // }
        let line_indicator = format!(
            "{}/{}",
            self.cursor_position.y.saturating_add(1),
            self.document.len()
        );
        let len = status.len()+line_indicator.len();
        if width >len {
            status.push_str(&" ".repeat(width-len))
        }
        status = format!("{}{}",status,line_indicator);
        status.truncate(width);
        Terminal::set_bg_color(STATUS_BG_COLOR);
        // println!("{}\r",spaces);
        // Terminal::reset_bg_color();
        Terminal::set_fg_color(STATUS_FG_COLOR);
        println!("{}\r",status);
        Terminal::reset_fg_color();
        Terminal::reset_bg_color();

    }
    fn draw_message_bar(&self){
        Terminal::clear_current_line();
    }


    
}

fn die(e: &std::io::Error) {
    // 发生错误时清空屏幕，打印错误
    Terminal::clear_screen();
    panic!("{}",e);
}
