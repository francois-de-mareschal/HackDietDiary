#[derive(Clone, Debug, PartialEq)]
pub(in crate::recordings) struct DayRecords {
    pub(in crate::recordings) weight: Option<f32>,
    pub(in crate::recordings) notes: Option<String>,
}
