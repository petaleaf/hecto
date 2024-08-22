use std::cmp;
use unicode_segmentation::UnicodeSegmentation;
pub struct Row {
    string: String,
    len: usize,
}

impl From<&str> for Row{
    fn from(slice: &str) ->Self {
        // Self {
            let mut row = Self{
                string: String::from(slice),
                len: 0,
            };
            row.update_len();
            row
        // }
    }
}

impl Row {
    pub fn render(&self,start:usize,end:usize) -> String {
        // 取最小值，避免越界访问
        let end = cmp::min(end,self.string.len());
        let start = cmp::min(start, end);
        // unwarp_or_default() 返回无效时返回默认的空（）
        // self.string.get(start..end).unwrap_or_default().to_string()
        // self.string.get(start..end).unwrap_or_default().to_string()
        let mut result = String::new();
        for grapheme in self.string[..]
            .graphemes(true)
            .skip(start)
            .take(end - start)
        {
            // result.push_str(grapheme);
            // 处理TAB键占据八个字符的情况
            if grapheme== "\t" {
                result.push_str(" ");
            }else {
                result.push_str(grapheme);
            }
        }
        result
    }
    // 获取水平方向的长度
    pub fn len(&self) ->usize{
        // self.string.len()
        // 按照我看看到的字计数，即多少字位
        // self.string[..].graphemes(true).count()
        self.len
    }
    pub fn is_empty(&self) ->bool{
        // self.string.is_empty()
        self.len == 0
    }
    // 在此处单独是实现，可以避免count便利整个迭代器，这样可以减少资源消耗
    fn update_len(&mut self){
        self.len = self.string[..].graphemes(true).count();
    }
}
