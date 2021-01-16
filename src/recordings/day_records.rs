#[derive(Clone, Debug, PartialEq)]
pub struct DayRecords {
    pub(in crate::recordings) weight: Option<f32>,
    pub(in crate::recordings) notes: Option<String>,
}

impl DayRecords {
    pub fn weight(&self) -> Option<f32> {
        return if let Some(weight) = self.weight {
            Some(weight)
        } else {
            None
        };
    }

    pub fn notes(&self) -> Option<&str> {
        return if let Some(notes) = &self.notes {
            Some(&notes[..])
        } else {
            None
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn weight_some_value() {
        let day_records = DayRecords {
            weight: Some(85.0),
            notes: None,
        };

        assert_eq!(day_records.weight(), Some(85.0));
    }

    #[test]
    fn weight_none_value() {
        let day_records = DayRecords {
            weight: None,
            notes: Some(String::from(
                "Terra tremuit et quievit, dum resurgeret judicio Deus, alleluia.",
            )),
        };

        assert_eq!(day_records.weight(), None);
    }

    #[test]
    fn notes_some_value() {
        let day_records = DayRecords {
            weight: None,
            notes: Some(String::from(
                "In principio erat Verbum, et Verbum erat apud Deum, et Deus erat Verbum.",
            )),
        };

        assert_eq!(
            day_records.notes(),
            Some("In principio erat Verbum, et Verbum erat apud Deum, et Deus erat Verbum.",)
        );
    }

    #[test]
    fn notes_none_value() {
        let day_records = DayRecords {
            weight: Some(85.0),
            notes: None,
        };

        assert_eq!(day_records.notes(), None)
    }
}
