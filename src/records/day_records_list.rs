use crate::records::day_records;
use chrono::prelude::*;
use std::collections::BTreeMap;

pub struct DayRecordsList {
    records: BTreeMap<DateTime<Utc>, day_records::DayRecords>,
}

impl DayRecordsList {
    pub fn add(&mut self, day_records: day_records::DayRecords, date: Option<DateTime<Utc>>) {
        todo!()
    }

    pub fn remove(&mut self, day_records: day_records::DayRecords, date: Option<DateTime<Utc>>) {
        todo!()
    }

    pub fn range(&self, from: Option<DateTime<Utc>>, to: Option<DateTime<Utc>>) {
        todo!()
    }
}

impl Iterator for DayRecordsList {
    type Item = day_records::DayRecords;

    fn next(&mut self) -> Option<day_records::DayRecords> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
