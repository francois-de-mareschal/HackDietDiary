#[derive(Clone, Debug, PartialEq)]
pub struct DayRecords {
    pub weight: Option<f32>,
    pub notes: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
}
