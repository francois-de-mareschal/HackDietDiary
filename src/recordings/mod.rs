mod day_records;
mod day_records_list;
mod report;

mod error {
    use chrono::{Date, Utc};
    use std::error::Error;
    use std::fmt::{self, Display};

    #[derive(Debug)]
    pub(crate) enum RecordingsError {
        DateAddingInTheFuture(Date<Utc>),
        DateRemovingInTheFuture(Date<Utc>),
        DayRecordsEmpty,
        NoDayRecordsAtDate(Date<Utc>),
        StartDateAfterEndDate {
            start_date: Date<Utc>,
            end_date: Date<Utc>,
        },
        ZeroOrNegativeWeight(f32),
    }

    impl Display for RecordingsError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match &self {
                RecordingsError::DateAddingInTheFuture(date) => fmt::write(
                    f,
                    format_args!("The date of the record to add is in the future: {}.", date),
                ),
                RecordingsError::DateRemovingInTheFuture(date) => fmt::write(
                    f,
                    format_args!(
                        "The date of the record to remove is in the future: {}.",
                        date
                    ),
                ),
                RecordingsError::DayRecordsEmpty => {
                    fmt::write(f, format_args!("Provided day records are empty."))
                }
                RecordingsError::NoDayRecordsAtDate(date) => fmt::write(
                    f,
                    format_args!("There are no records to remove for this date: {}.", date),
                ),
                RecordingsError::StartDateAfterEndDate {
                    start_date,
                    end_date,
                } => fmt::write(
                    f,
                    format_args!(
                        "The start date {} is before the end date {}.",
                        start_date, end_date
                    ),
                ),
                RecordingsError::ZeroOrNegativeWeight(weight) => fmt::write(
                    f,
                    format_args!("The provided weight is zero or negative: {}.", weight),
                ),
            }
        }
    }

    impl Error for RecordingsError {}
}
