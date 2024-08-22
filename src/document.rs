use crate::row;
use crate::Position;
use crate::Row;
use std::fs;
#[derive(Default)]

pub struct Document {
    rows: Vec<Row>,
    pub file_name: Option<String>,
}

impl Document {
    // pub fn open()->Self {
    pub fn open(filename:&str) ->Result<Self, std::io::Error> {
        let contents = fs::read_to_string(filename)?;
        let mut rows = Vec::new();
        // rows.push(Row::from(""));
        // Self { rows }
        for value in contents.lines() {
            rows.push(Row::from(value));
        }
        Ok(Self {
            rows,
            file_name: Some(filename.to_string()),
        })
        
    }
    pub fn row(&self, index:usize) -> Option<&Row> {
        self.rows.get(index)
    }
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
    pub fn len(&self) ->usize {
        self.rows.len()
    }
    pub fn insert(&mut self, at:&Position,c:char) {
        if at.y == self.len(){
            // 如果在最后一行则添加新行
            let mut row = Row::default();
            row.insert(0,c);
            self.rows.push(row);

        }else if at.y < self.len(){
            let row = self.rows.get_mut(at.y).unwrap();
            // 在光标当前行插入
            row.insert(at.x, c)
        }
    }
    pub fn delet(&mut self,at:&Position){
        let len = self.len();
        if at.y >=len{
            return;
        }
        // let row = self.rows.get_mut(at.y).unwrap();
        // row.delet(at.x);
        if at.x == self.rows.get_mut(at.y).unwrap().len() && at.y<len - 1 {
            let next_row = self.rows.remove(at.y+1);
            let row = self.rows.get_mut(at.y).unwrap();
            row.append(&next_row);
        }else{
            let row = self.rows.get_mut(at.y).unwrap();
            row.delet(at.x)
        }
    }
}