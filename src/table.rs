use tabled::Table;

pub trait AsTableRow {
    const HEADERS: &'static [&'static str];

    fn as_row(&self) -> Vec<String>;
}

pub trait AsTable {
    fn as_table(&self) -> Table;
}

impl<T: AsTableRow> AsTable for Vec<T> {
    fn as_table(&self) -> Table {
        let mut builder = tabled::builder::Builder::default();
        builder.push_record(T::HEADERS.iter().copied());

        for item in self.iter() {
            builder.push_record(item.as_row());
        }

        builder.build()
    }
}
