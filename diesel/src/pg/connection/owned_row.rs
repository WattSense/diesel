use super::result::PgResult;
use super::row::PgField;
use crate::pg::Pg;
use crate::row::*;

#[allow(missing_debug_implementations)]
pub struct OwnedPgRow {
    db_result: PgResult,
    row_idx: usize,
}

impl OwnedPgRow {
    pub(crate) fn new(db_result: PgResult, row_idx: usize) -> Self {
        OwnedPgRow { db_result, row_idx }
    }
}

impl RowSealed for OwnedPgRow {}

impl<'a> Row<'a, Pg> for OwnedPgRow {
    type Field<'f> = PgField<'f> where 'a: 'f, Self: 'f;
    type InnerPartialRow = Self;

    fn field_count(&self) -> usize {
        self.db_result.column_count()
    }

    fn get<'b, I>(&'b self, idx: I) -> Option<Self::Field<'b>>
    where
        'a: 'b,
        Self: RowIndex<I>,
    {
        let idx = self.idx(idx)?;
        Some(PgField::new(&self.db_result, self.row_idx, idx))
    }

    fn partial_row(&self, range: std::ops::Range<usize>) -> PartialRow<'_, Self::InnerPartialRow> {
        PartialRow::new(self, range)
    }
}

impl RowIndex<usize> for OwnedPgRow {
    fn idx(&self, idx: usize) -> Option<usize> {
        if idx < self.field_count() {
            Some(idx)
        } else {
            None
        }
    }
}

impl<'a> RowIndex<&'a str> for OwnedPgRow {
    fn idx(&self, field_name: &'a str) -> Option<usize> {
        (0..self.field_count()).find(|idx| self.db_result.column_name(*idx) == Some(field_name))
    }
}
