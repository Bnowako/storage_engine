pub mod mem_table {
    pub struct MemTable {
        pub values: Vec<Pair>,
    }

    pub fn new() -> MemTable {
        MemTable {
            values: Vec::new()
        }
    }

    pub struct Pair {
        pub(crate) key: String,
        pub(crate) value: Vec<u8>,
    }


    impl MemTable {
        pub fn add(&mut self, pair: Pair) {
            match self.values.binary_search_by(|value| value.key.cmp(&pair.key)) {
                Ok(pos) => {}
                Err(pos) => { self.values.insert(pos, pair) }
            }
        }
        pub fn get(&self, key: String) -> Option<&Pair> {
            return match self.values.binary_search_by(|value| value.key.cmp(&key)) {
                Ok(pos) => { self.values.get(pos) }
                Err(pos) => { None }
            };
        }
    }
}

use crate::mem_table::mem_table::{new, Pair};

#[test]
fn add_and_get() {
    let mut mem_table = new();
    mem_table.add(Pair { key: String::from("a"), value: vec![1, 4, 3] });
    mem_table.add(Pair { key: String::from("b"), value: vec![1, 4, 3] });

    let option = mem_table.get(String::from("a"));
    assert_eq!(option.expect("a").key, String::from("a"))
}


