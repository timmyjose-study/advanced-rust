mod command {

    type DbFnPtr = fn() -> String;

    struct Command {
        execute: DbFnPtr,
        rollback: DbFnPtr,
    }

    pub struct Schema {
        commands: Vec<Command>,
    }

    impl Schema {
        pub fn new() -> Self {
            Schema {
                commands: Vec::new(),
            }
        }

        pub fn add_migration(&mut self, execute: DbFnPtr, rollback: DbFnPtr) {
            self.commands.push(Command { execute, rollback });
        }

        pub fn execute(&self) -> Vec<String> {
            self.commands.iter().map(|c| (c.execute)()).collect()
        }

        pub fn rollback(&self) -> Vec<String> {
            self.commands.iter().rev().map(|c| (c.rollback)()).collect()
        }
    }

    pub fn add_field() -> String {
        "add field".to_string()
    }

    pub fn drop_field() -> String {
        "remove field".to_string()
    }
}

fn main() {
    use command::*;

    let mut schema = Schema::new();
    schema.add_migration(|| "create table".to_string(), || "drop table".to_string());
    schema.add_migration(add_field, drop_field);

    assert_eq!(vec!["create table", "add field"], schema.execute());
    assert_eq!(vec!["remove field", "drop table"], schema.rollback());
}
