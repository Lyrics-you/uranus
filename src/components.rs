use crate::utils::backup_from_origin;
use crate::utils::{delete_regex, replace_regex, rollback_to_origin};

#[derive(Default)]
pub struct Charism<'a> {
    pub group: &'a str,
    pub name: &'a str,
    pub description: &'a str,
    strategy: Rc<RefCell<Strategy>>,
    backup_files: Rc<RefCell<Vec<String>>>,
    backup_home: &'a str,
}

impl<'a> Charism<'a> {
    pub fn new(group: &'a str, name: &'a str, description: &'a str) -> Self {
        Charism {
            group,
            name,
            description,
            strategy: Rc::new(RefCell::new(Strategy::new("./Eden"))),
            backup_files: Rc::new(RefCell::new(Vec::new())),
            backup_home: "./Eden",
        }
    }

    pub fn set_backup_home(&mut self, backup_home: &'a str) {
        self.backup_home = backup_home;
    }

    pub fn to_owned(&mut self) -> Self {
        Self {
            group: self.group,
            name: self.name,
            description: self.description,
            strategy: self.strategy.clone(),
            backup_files: self.backup_files.clone(),
            backup_home: self.backup_home,
        }
    }

    pub fn show(&self) {
        println!(
            "The {} belongs to the {} for {}",
            self.name, self.group, self.description
        );
    }

    pub fn add<'b>(&'a self, pitho: Rc<dyn Applyable + 'static>) -> &Charism<'a>
    where
        'b: 'static + 'a,
    {
        let backup_file_path = self.backup_home;
        let binding = self.strategy.clone();
        let mut strages = binding.borrow_mut();
        strages.add(pitho.clone());

        let file_path = pitho.get_file_path().to_string();

        let binding = self.backup_files.clone();
        let mut backup_files = binding.borrow_mut();

        if !backup_files.contains(&file_path) {
            backup_files.push(file_path);
            match pitho.get_type() {
                ApplyType::Replace | ApplyType::Delete => {
                    // backup origin file if not backup before
                    let file_path = pitho.get_file_path().to_string();

                    if backup_files.contains(&file_path) {
                        // rollback is the path checker, here will be ok
                        backup_from_origin(&file_path, backup_file_path).unwrap();
                    }
                }
                _ => {}
            }
        }
        self.to_owned()
    }

    pub fn apply(&'a self) -> Result<(), Box<dyn Error>> {
        // println!("<-{}->", self.group);
        let binding = self.strategy.clone();
        let strages: &Strategy = &binding.borrow();
        for strage in strages {
            if let ApplyType::RollBack = strage.get_type() {
                // rollback()
                self.rollback(false)?;
            } else {
                // apply changes
                match strage.do_apply() {
                    Ok(_) => {
                        let message = format!(
                            "[{}]({}) apply {:?} success!",
                            self.name,
                            strage.get_file_path(),
                            strage.get_type()
                        );
                        log::info!("{}", message);
                    }
                    Err(err) => {
                        let message = format!(
                            "[{}]({}) apply {:?} failed, {}",
                            self.name,
                            strage.get_file_path(),
                            strage.get_type(),
                            err
                        );
                        log::error!("{}",message);
                        self.rollback(true)?;
                        return Err(err);
                    }
                }
            }
        }
        // println!("<-{}->", self.group);
        Ok(())
    }

    pub fn rollback(&self, is_over: bool) -> Result<(), Box<dyn Error>> {
        let backup_file_path = self.backup_home;
        let binding = self.backup_files.clone();
        let backup_files: &Vec<String> = &binding.borrow();
        for file in backup_files {
            match rollback_to_origin(backup_file_path, &file) {
                Ok(_) => {
                    let message =
                        format!("[{}]({}) apply {} success!", self.name, file, "RollBack");
                    log::info!("{}", message);
                }
                Err(err) => {
                    let message = format!(
                        "[{}]({}) apply {} failed, {}",
                        self.name, file, "RollBack", err
                    );
                    log::error!("{}",message);
                    // self.rollback(true)?;
                    return Err(err);
                }
            }
            // println!("[{}]({}) apply {} success!", self.name, file, "RollBack");
        }
        if is_over {
            log::info!("<-{}->", self.name);
        }
        Ok(())
    }
}

use std::cell::RefCell;
use std::error::Error;
use std::fmt;
use std::rc::Rc;

#[derive(Debug)]
pub struct CustomError {
    pub message: String,
}

impl CustomError {
    pub fn new(message: &str) -> Box<dyn Error> {
        Box::new(CustomError {
            message: message.to_string(),
        })
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for CustomError {}

#[derive(Debug)]
pub enum ApplyType {
    Backup,
    RollBack,
    Replace,
    Delete,
}

pub trait Applyable {
    fn do_apply(&self) -> Result<(), Box<dyn Error>>;
    fn get_type(&self) -> ApplyType;
    fn get_file_path(&self) -> &str;
}

pub struct Backup<'a> {
    pub file_path: String,
    pub to: &'a str,
}

impl Backup<'_> {
    pub fn new<'a>(file_path: String, to: &'a str) -> Rc<Backup<'a>> {
        Rc::new(Backup { file_path, to })
    }
}

impl Applyable for Backup<'_> {
    fn do_apply(&self) -> Result<(), Box<dyn Error>> {
        backup_from_origin(self.get_file_path(), self.to)
    }
    fn get_type(&self) -> ApplyType {
        ApplyType::Backup
    }
    fn get_file_path(&self) -> &str {
        &self.file_path
    }
}

pub struct RollBack {
    pub file_path: String,
}

impl Applyable for RollBack {
    fn do_apply(&self) -> Result<(), Box<dyn Error>> {
        return Err(CustomError::new("needs to rollback before apply"));
    }

    fn get_type(&self) -> ApplyType {
        ApplyType::RollBack
    }
    fn get_file_path(&self) -> &str {
        &self.file_path
    }
}

impl RollBack {
    pub fn new(file_path: String) -> Rc<RollBack> {
        Rc::new(RollBack { file_path })
    }
}

pub struct Replace<'a> {
    pub file_path: String,
    pub from: &'a str,
    pub to: &'a str,
}

impl Replace<'_> {
    pub fn new<'a>(file_path: String, from: &'a str, to: &'a str) -> Rc<Replace<'a>> {
        Rc::new(Replace {
            file_path,
            from,
            to,
        })
    }
}

impl Applyable for Replace<'_> {
    fn do_apply(&self) -> Result<(), Box<dyn Error>> {
        replace_regex(&self.file_path, &self.from, &self.to)
    }
    fn get_type(&self) -> ApplyType {
        ApplyType::Replace
    }
    fn get_file_path(&self) -> &str {
        &self.file_path
    }
}

pub struct Delete<'a> {
    pub file_path: String,
    pub from: &'a str,
}

impl Delete<'_> {
    pub fn new<'a>(file_path: String, from: &'a str) -> Rc<Delete<'a>> {
        Rc::new(Delete { file_path, from })
    }
}

impl Applyable for Delete<'_> {
    fn do_apply(&self) -> Result<(), Box<dyn Error>> {
        delete_regex(&self.file_path, &self.from)
    }
    fn get_type(&self) -> ApplyType {
        ApplyType::Delete
    }
    fn get_file_path(&self) -> &str {
        &self.file_path
    }
}

#[derive(Default)]
pub struct Strategy {
    pub pithos: Vec<Rc<dyn Applyable>>,
}

impl Strategy {
    pub fn new(backup_home: &'static str) -> Strategy {
        Strategy {
            pithos: vec![RollBack::new(backup_home.to_string())],
        }
    }

    pub fn add(&mut self, pitho: Rc<dyn Applyable>) -> &mut Strategy {
        self.pithos.push(pitho);
        self
    }
}

// impl IntoIterator for Strategy {
//     type Item = Box<dyn Applyable>;
//     type IntoIter = std::vec::IntoIter<Self::Item>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.pithos.into_iter()
//     }
// }

impl<'a> IntoIterator for &'a Strategy {
    type Item = &'a Rc<dyn Applyable>;
    type IntoIter = std::slice::Iter<'a, Rc<dyn Applyable>>;

    fn into_iter(self) -> Self::IntoIter {
        self.pithos.iter()
    }
}
