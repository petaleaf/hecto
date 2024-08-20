use std::cmp;
pub struct Row {
    string: String,
}

impl From<&str> for Row{
    fn from(slice: &str) ->Self {
        Self {
            string: String::from(slice),
        }
    }
}

impl Row {
    pub fn render(&self,start:usize,end:usize) -> String {
        // 取最小值，避免越界访问
        let end = cmp::min(end,self.string.len());
        let start = cmp::min(start, end);
        // unwarp_or_default() 返回无效时返回默认的空（）
        // self.string.get(start..end).unwrap_or_default().to_string()
        self.string.get(start..end).unwrap_or_default().to_string()
    }
    // 获取水平方向的长度
    pub fn len(&self) ->usize{
        self.string.len()
    }
    pub fn is_empty(&self) ->bool{
        self.string.is_empty()
    }
}
