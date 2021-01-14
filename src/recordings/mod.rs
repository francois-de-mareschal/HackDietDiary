mod day_records;
mod day_records_list;
mod report;

mod error {
    use chrono::prelude::*;
    use std::error::Error;
    use std::fmt::{self, Display};

    #[derive(Debug)]
    enum RecordingsError {
        DateInTheFuture(Date<Utc>),
        DayRecordsEmpty,
        NoDayRecordsAtDate(Date<Utc>),
        StartDateAfterEndDate {
            start_date: Date<Utc>,
            end_date: Date<Utc>,
        },
        ZeroOrNegativeWeight(f32),
    }

    impl Display for RecordingsError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match &self {
                RecordingsError::DateInTheFuture(date) => todo!(),
                RecordingsError::DayRecordsEmpty => todo!(),
                RecordingsError::NoDayRecordsAtDate(date) => todo!(),
                RecordingsError::StartDateAfterEndDate {
                    start_date,
                    end_date,
                } => todo!(),
                RecordingsError::ZeroOrNegativeWeight(weight) => todo!(),
            }
        }
    }

    impl Error for RecordingsError {}
}
