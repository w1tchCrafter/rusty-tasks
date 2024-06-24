use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::io::Error;
use std::result::Result;

use comfy_table::Table;
use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize, Debug)]
struct Task {
    text: String,
    number: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Tasks {
    completed: Vec<Task>,
    todo: Vec<Task>,
    #[serde(skip)]
    path: String,
}

impl Tasks {
    pub fn new(path: &str) -> Self {
        if let Ok(task_str) = fs::read_to_string(path) {
            let mut json_tasks: Tasks = serde_json::from_str(&task_str).unwrap();
            json_tasks.path = path.to_string();
            json_tasks
        } else {
            let file = File::create(path).unwrap();
            let mut writer = BufWriter::new(file);
            let json_tasks = Self {
                completed: Vec::new(),
                todo: Vec::new(),
                path: path.to_string()
            };

            if let Err(err) = serde_json::to_writer(&mut writer, &json_tasks) {
                panic!("{:?}", err);
            }

            json_tasks
        }
    }

    pub fn add_task(&mut self, t: &str) -> Result<(), Error> {
        self.todo.push(Task {
            text: t.to_string(),
            number: self.todo.len() as u32,
        });

        self.write_json()?;

        Ok(())
    }

    pub fn remove_task(&mut self, task_num: u32) -> Result<(), Error> {
        if self.todo.len() != 0 {
            if task_num > (self.todo.len() - 1).try_into().unwrap() {
                ()
            }
        }

        let rm: Vec<Task> = self
            .todo
            .drain(..)
            .filter(|t| t.number != task_num)
            .collect();

        self.todo = rm;
        self.write_json()?;

        Ok(())
    }

    fn write_json(&self) -> Result<(), Error> {
        let json = serde_json::to_string(&self)?;
        fs::write(&self.path, &json).expect("unable to edit tasks file");

        Ok(())
    }

    pub fn list_tasks(&self) {
        let mut table = Table::new();
        table.set_header(vec!["#", "task"]);

        self.todo.iter().for_each(|item| {
            table.add_row(vec![item.number.to_string(), item.text.clone()]);
        });

        println!("{table}");
    }
}
