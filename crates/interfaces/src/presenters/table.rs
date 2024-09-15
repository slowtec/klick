use std::collections::HashMap;

use klick_domain::{Value, ValueId as Id};

pub trait TablePresenter {
    fn present_table(&self, values: HashMap<Id, Value>, sections: Vec<(String, Vec<Id>)>) -> Table;
}

pub struct Table {
    pub sections: Vec<TableSection>,
}

pub struct TableSection {
    pub title: String,
    pub rows: Vec<TableRow>,
}

pub struct TableRow {
    pub id: Id,
    pub label: String,
    pub value: Option<String>,
    pub unit: Option<String>,
}
