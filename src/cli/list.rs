use crate::config::Config;
use crate::error::Result;
use crate::exec::dialog_info::DialogInfo;
use crate::exec::restic_list::ResticList;

/// A table formatter
#[derive(Debug, Clone)]
struct TableFormatter {
    /// The fields
    fields: Vec<String>,
    /// The amount of columns within the table
    columns: usize,
}
impl TableFormatter {
    /// Creates a new table formatter
    pub const fn new(columns: usize) -> Self {
        Self { fields: Vec::new(), columns }
    }
    /// Pushes a row of fields to the table
    pub fn push<I, F>(&mut self, row: I) -> Result
    where
        I: IntoIterator<Item = F>,
        F: ToString,
    {
        // Validate the amount of fields
        let mut fields: Vec<_> = row.into_iter().map(|f| f.to_string()).collect();
        if fields.len() != self.columns {
            Err(einval!("Invalid amount of fields in row (expected {}; got {})", self.columns, fields.len()))?;
        }

        // Append the fields
        self.fields.append(&mut fields);
        Ok(())
    }

    /// Formats the table
    pub fn format(mut self) -> Vec<Vec<String>> {
        // Format the table
        for column in 0..self.columns {
            let max_len = self.column_max_len(column);
            self.column_rpad(column, max_len);
        }

        // Split the fields into real columns
        let mut columns = Vec::new();
        while !self.fields.is_empty() {
            let row: Vec<_> = self.fields.drain(0..self.columns).collect();
            columns.push(row);
        }
        columns
    }
    /// Get the length of the longest field of a given `column`
    fn column_max_len(&self, column: usize) -> usize {
        self.fields.iter().skip(column).step_by(self.columns).map(|f| f.chars().count()).max().unwrap_or_default()
    }
    /// Right-pads each field for the given `column` to `max_len`
    fn column_rpad(&mut self, column: usize, max_len: usize) {
        // Create an iterator for the fields for the given column
        let fields = self.fields.iter_mut().skip(column).step_by(self.columns);

        // Pad each field
        for field in fields {
            let field_len = field.chars().count();
            for _ in 0..max_len.saturating_sub(field_len) {
                field.push(' ');
            }
        }
    }
}

/// Lists all archives
pub struct List {
    /// The config
    config: Config,
}
impl List {
    /// Creates a command to list all archives
    pub const fn new(config: Config) -> Self {
        Self { config }
    }

    /// Executes the command
    pub fn exec(self) -> Result {
        // Print the status info
        DialogInfo::new("Listing snapshots...")?.exec()?;
        println!();

        // Collect all archives
        let mut table = TableFormatter::new(4);
        for archive in ResticList::new(&self.config)?.exec()? {
            table.push([archive.short_id, archive.time, archive.tags.join(","), archive.hostname])?;
        }

        // Print all archives
        for row in table.format() {
            println!("{}  @{}  #{}  [{}]", row[0], row[1], row[2], row[3]);
        }
        Ok(())
    }
}
