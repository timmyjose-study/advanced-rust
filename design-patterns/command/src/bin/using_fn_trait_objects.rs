mod command {

    type Migration<'a> = Box<dyn Fn() -> &'a str>;

    pub struct Schema<'a> {
        executes: Vec<Migration<'a>>,
        rollbacks: Vec<Migration<'a>>,
    }

    impl<'a> Schema<'a> {
        pub fn new() -> Self {
            Schema {
                executes: Vec::new(),
                rollbacks: Vec::new(),
            }
        }

        pub fn add_migration<E, R>(&mut self, execute: E, rollback: R)
        where
            E: Fn() -> &'a str + 'static,
            R: Fn() -> &'a str + 'static,
        {
            self.executes.push(Box::new(execute));
            self.rollbacks.push(Box::new(rollback));
        }

        pub fn execute(&self) -> Vec<&str> {
            self.executes.iter().map(|c| c()).collect()
        }

        pub fn rollback(&self) -> Vec<&str> {
            self.rollbacks.iter().rev().map(|c| c()).collect()
        }
    }
}

fn main() {
    use command::*;

    let mut schema = Schema::new();
    schema.add_migration(|| "create table", || "drop table");
    schema.add_migration(|| "add field", || "remove field");

    assert_eq!(vec!["create table", "add field"], schema.execute());
    assert_eq!(vec!["remove field", "drop table"], schema.rollback());
}
