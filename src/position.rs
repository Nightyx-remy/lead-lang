use std::fmt::{Debug, Formatter};

pub struct Positioned<A> {
    pub start: Position,
    pub end: Position,
    pub data: A
}

impl<A> Positioned<A> {

    pub fn new(data: A, start: Position, end: Position) -> Self {
        return Self {
            start,
            end,
            data,
        };
    }

    pub fn eof(data: A) -> Self {
        return Self {
            start: Position::new(usize::MAX, 0, 0),
            end: Position::new(usize::MAX, 0, 0),
            data
        }
    }

    pub fn show_on_text(&self, src: String) {
        if self.start.index == usize::MAX {
            let line = src.lines().last().unwrap_or("a");
            println!("{}\n{}^", line, " ".repeat(line.len()));
        } else {
            let lines: Vec<&str> = src.lines().collect();

            for i in self.start.line..=self.end.line {
                let start = if i == self.start.line { self.start.column } else { 0 };
                let end = if i == self.end.line { self.end.column } else { lines[i - 1].len() };

                println!("{}", lines[i - 1]);
                println!("{}{}", " ".repeat(start), "^".repeat(end - start));
            }
        }
    }

    pub fn convert<B>(&self, data: B) -> Positioned<B> {
        return Positioned {
            start: self.start.clone(),
            end: self.end.clone(),
            data
        }
    }

}

impl<A: Debug> Debug for Positioned<A> {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.data.fmt(f)
    }

}

impl<A: Clone> Clone for Positioned<A> {
    fn clone(&self) -> Self {
        return Self {
            start: self.start.clone(),
            end: self.end.clone(),
            data: self.data.clone(),
        };
    }
}

#[derive(Clone)]
pub struct Position {
    pub index: usize,
    pub line: usize,
    pub column: usize
}

impl Position {

    pub fn new(index: usize, line: usize, column: usize) -> Self {
        return Self {
            index,
            line,
            column,
        };
    }

    pub fn advance(&mut self, chr: char) {
        self.index += 1;
        if chr == '\n' {
            self.line += 1;
            self.column = 0;
        } else {
            self.column += 1;
        }
    }

}