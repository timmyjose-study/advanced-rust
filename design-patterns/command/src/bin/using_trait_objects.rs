mod command {
    pub trait Migration {
        fn execute(&self) -> &str;
        fn rollback(&self) -> &str;
    }

    pub struct CreateTable;

    impl Migration for CreateTable {
        fn execute(&self) -> &str {
            "create table"
        }

        fn rollback(&self) -> &str {
            "drop table"
        }
    }

    pub struct AddField;

    impl Migration for AddField {
        fn execute(&self) -> &str {
            "add field"
        }

        fn rollback(&self) -> &str {
            "remove field"
        }
    }

    pub struct Schema {
        commands: Vec<Box<dyn Migration>>,
    }

    impl Schema {
        pub fn new() -> Self {
            Schema {
                commands: Vec::new(),
            }
        }

        pub fn add_migration(&mut self, cmd: Box<dyn Migration>) {
            self.commands.push(cmd);
        }

        pub fn execute(&self) -> Vec<&str> {
            self.commands.iter().map(|c| c.execute()).collect()
        }

        pub fn rollback(&self) -> Vec<&str> {
            self.commands.iter().rev().map(|c| c.rollback()).collect()
        }
    }
}

fn main() {
    use command::*;

    let mut schema = Schema::new();
    let cmd = Box::new(CreateTable);
    schema.add_migration(cmd);
    let cmd = Box::new(AddField);
    schema.add_migration(cmd);

    assert_eq!(vec!["create table", "add field"], schema.execute());
    assert_eq!(vec!["remove field", "drop table"], schema.rollback());
}