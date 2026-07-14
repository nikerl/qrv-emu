// Copyright 2026
// Apache License, Version 2.0, see LICENSE for details.
//
// Author: Nik Erlandsson

use std::{
    collections::HashMap, 
    fs::File
};

pub struct FileTable {
    pub files: HashMap<i32, File>,
}

impl FileTable {
    pub fn new() -> Self {
        return FileTable{files: HashMap::new()};
    }

    pub fn insert(&mut self, file: File) -> i32 {
        let mut fd = 3; // 0-2 is reserved for stdin, stdout, stderr
        while self.files.contains_key(&fd) {
            fd += 1;
        }
        self.files.insert(fd, file);
        
        return fd;
    }

    /// Returns 0 if successfull, -1 if fd not found
    pub fn remove(&mut self, fd: i32) -> i32 {
        if fd == 0 || fd == 1 || fd == 2 { // stdin, stdout, stderr are not a real files
            return 0;
        }

        match self.files.remove(&fd) {
            Some(_) => return 0, // success
            None => { // fd not found
                println!("fd: {} not found", fd);
                return -1
            }
        }
    }
}
