use std::{
    fmt::Debug,
    ops::{Add, BitOr, Range},
    slice, vec,
};

use rand::Rng;

pub struct PaintedBoard {
    width: usize,
    height: usize,
    cells: Vec<bool>,
}

impl PaintedBoard {
    pub fn new_random(rng: &mut impl Rng, width: usize, height: usize, p: f64) -> Self {
        let cells: Vec<bool> = (0..width * height).map(|_| rng.random_bool(p)).collect();
        Self {
            width,
            height,
            cells,
        }
    }
}

const BLACK_COLOR: &str = "░░";
const WHITE_COLOR: &str = "██";
const UNKNOWN_COLOR: &str = "..";

impl Debug for PaintedBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let column_infos = self.get_column_infos();

        let max_size_rows: usize = column_infos
            .rows
            .iter()
            .map(|row| {
                row.info
                    .iter()
                    .map(|&value| value.ilog10() + 2)
                    .sum::<u32>()
            })
            .max()
            .unwrap() as usize;

        for k in 0..self.width * self.height {
            if k % self.width == 0 {
                if k != 0 {
                    write!(f, "\n")?;
                }

                let row = column_infos.rows[k / self.width]
                    .info
                    .iter()
                    .map(|&id| id.to_string() + " ")
                    .collect::<String>();

                write!(f, "{row:>max_size_rows$}", max_size_rows = max_size_rows)?;
            }

            if self.cells[k] {
                write!(f, "{}", WHITE_COLOR)?;
            } else {
                write!(f, "{}", BLACK_COLOR)?;
            }
        }
        Ok(())
    }
}

pub struct PaintedColumn {
    cells: Vec<bool>,
}

impl PaintedColumn {
    pub fn new_random(rng: &mut impl Rng, p: f64, lenght: usize) -> Self {
        let cells: Vec<bool> = (0..lenght).map(|_| rng.random_bool(p)).collect();

        Self { cells }
    }
}

impl PaintedBoard {
    pub fn get_row(&self, j: usize) -> PaintedColumn {
        let cells = self.cells[j * self.width..(j + 1) * self.width].to_vec();

        PaintedColumn { cells }
    }

    pub fn get_column(&self, i: usize) -> PaintedColumn {
        let cells = self.cells[i..self.height * self.width]
            .iter()
            .step_by(self.width)
            .copied()
            .collect();
        PaintedColumn { cells }
    }
}

pub struct ColumnInfo {
    info: Vec<usize>,
}

impl ColumnInfo {
    pub fn new(info: Vec<usize>) -> Self {
        ColumnInfo { info }
    }
}

impl Debug for ColumnInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.info)
    }
}

impl PaintedColumn {
    pub fn get_info(&self) -> ColumnInfo {
        let mut state = 0;
        let mut info = vec![];

        for cell in &self.cells {
            match (state, cell) {
                (_, true) => {
                    state += 1;
                }
                (0, false) => {
                    1;
                }
                (_, false) => {
                    info.push(state);
                    state = 0;
                }
            }
        }

        if state != 0 {
            info.push(state);
        }

        ColumnInfo { info }
    }
}

pub struct ColumnInfos {
    columns: Vec<ColumnInfo>,
    rows: Vec<ColumnInfo>,
}

impl ColumnInfos {
    pub fn new(columns: Vec<ColumnInfo>, rows: Vec<ColumnInfo>) -> Self {
        ColumnInfos { columns, rows }
    }
}

impl PaintedBoard {
    pub fn get_column_infos(&self) -> ColumnInfos {
        let columns = (0..self.width)
            .map(|i| self.get_column(i).get_info())
            .collect();
        let rows = (0..self.height)
            .map(|j| self.get_row(j).get_info())
            .collect();
        ColumnInfos { columns, rows }
    }
}

pub struct Board {
    width: usize,
    height: usize,
    cells: Vec<Option<bool>>,
    infos: ColumnInfos,
}

impl From<ColumnInfos> for Board {
    fn from(infos: ColumnInfos) -> Self {
        let width = infos.columns.len();
        let height = infos.rows.len();

        Board {
            width,
            height,
            cells: vec![None; width * height],
            infos,
        }
    }
}

impl PaintedBoard {
    pub fn into_empty_board(&self) -> Board {
        Board {
            width: self.width,
            height: self.height,
            cells: vec![None; self.width * self.height],
            infos: self.get_column_infos(),
        }
    }
}

pub struct Column {
    cells: Vec<Option<bool>>,
}

impl From<PaintedColumn> for Column {
    fn from(value: PaintedColumn) -> Self {
        Column {
            cells: value.cells.iter().map(|&cell| Some(cell)).collect(),
        }
    }
}

impl Column {
    pub fn new(cells: Vec<Option<bool>>) -> Column {
        Column { cells }
    }

    pub fn new_ramdom_from(column: &PaintedColumn, rng: &mut impl Rng, p: f64) -> Column {
        let cells = column
            .cells
            .iter()
            .map(|&cell| if rng.random_bool(p) { Some(cell) } else { None })
            .collect();
        Column { cells }
    }

    pub fn painted_rate(&self) -> f32 {
        self.cells.iter().filter(|&cell| cell.is_some()).count() as f32 / self.cells.len() as f32
    }

    pub fn verify(self, column: PaintedColumn) -> bool {
        self.fit_in(&column.into())
    }
}

impl Board {
    pub fn get_row(&self, j: usize) -> Column {
        let cells = self.cells[j * self.width..(j + 1) * self.width].to_vec();
        Column { cells }
    }

    pub fn get_column(&self, i: usize) -> Column {
        let cells = self.cells[i..self.height * self.width]
            .iter()
            .step_by(self.width)
            .copied()
            .collect();
        Column { cells }
    }

    pub fn set_row(&mut self, j: usize, row: Column) {
        for index in 0..self.width {
            self.cells[index + j * self.width] = row.cells[index];
        }
    }

    pub fn set_column(&mut self, i: usize, column: Column) {
        for index in 0..self.width {
            self.cells[i + index * self.width] = column.cells[index];
        }
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_size_rows: usize = self
            .infos
            .rows
            .iter()
            .map(|row| {
                row.info
                    .iter()
                    .map(|&value| value.ilog10() + 2)
                    .sum::<u32>()
            })
            .max()
            .unwrap() as usize;

        for k in 0..self.width * self.height {
            if k % self.width == 0 {
                if k != 0 {
                    write!(f, "\n")?;
                }

                let row = self.infos.rows[k / self.width]
                    .info
                    .iter()
                    .map(|&id| id.to_string() + " ")
                    .collect::<String>();

                write!(f, "{row:>max_size_rows$}", max_size_rows = max_size_rows)?;
            }

            match self.cells[k] {
                None => write!(f, "{:}", UNKNOWN_COLOR)?,
                Some(true) => write!(f, "{:}", WHITE_COLOR)?,
                Some(false) => write!(f, "{:}", BLACK_COLOR)?,
            }
        }
        Ok(())
    }
}

impl Debug for Column {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for cell in &self.cells {
            match cell {
                None => write!(f, "{:}", UNKNOWN_COLOR)?,
                Some(true) => write!(f, "{:}", WHITE_COLOR)?,
                Some(false) => write!(f, "{:}", BLACK_COLOR)?,
            }
        }
        Ok(())
    }
}

// impl Column {
//     fn paint_by_info(&mut self, info: &ColumnInfo) {
//         assert!(
//             self.cells.len() + 1 >= info.info.iter().sum::<usize>() + info.info.len(),
//             "{:?} {:?}",
//             self.cells,
//             info.info
//         );
//         let width = self.cells.len() + 1;
//         let space_used = info.info.iter().sum::<usize>() + info.info.len();
//         assert!(width >= space_used);
//         let remove = width - space_used;
//         let mut index = 0;

//         if info.info.len() == 0 {
//             for index in 0..self.cells.len() {
//                 self.cells[index] = Some(false);
//             }
//         } else {
//             for &num in &info.info {
//                 for i in remove..num {
//                     self.cells[index + i] = Some(true);
//                 }

//                 index += num + 1;

//                 if remove == 0 && index < width {
//                     self.cells[index - 1] = Some(false);
//                 }
//             }
//         }
//     }

//     fn map_on_range(&mut self, range: Range<usize>, map: impl Fn(&mut Column)) {
//         let mut column = Column {
//             cells: self.cells[range.clone()].to_vec(),
//         };
//         map(&mut column);
//         for i in range.clone() {
//             self.cells[i] = column.cells[i - range.start];
//         }
//     }

//     fn simplify(
//         &mut self,
//         info: &ColumnInfo,
//         range: Range<usize>,
//     ) -> Option<(Range<usize>, ColumnInfo)> {
//         let mut new_info = info.info.clone();
//         let mut start_index = range.start;
//         let mut end_index = range.end;

//         while start_index < end_index {
//             let Some(value) = self.cells[start_index] else {
//                 break;
//             };

//             start_index += 1;

//             if value {
//                 let quant = new_info.remove(0);
//                 for i in start_index..start_index + quant - 1 {
//                     self.cells[i] = Some(true);
//                 }
//                 start_index += quant - 1;
//                 if start_index < end_index {
//                     self.cells[start_index] = Some(false);
//                     start_index += 1;
//                 }
//             }
//         }

//         if start_index >= end_index {
//             return None;
//         }

//         while start_index < end_index {
//             let Some(value) = self.cells[end_index - 1] else {
//                 break;
//             };

//             end_index -= 1;

//             if value {
//                 let quant = new_info.pop().unwrap() - 1;
//                 for i in end_index - quant..end_index {
//                     self.cells[i] = Some(true);
//                 }
//                 end_index -= quant;
//                 if end_index < start_index {
//                     end_index -= 1;
//                     self.cells[end_index] = Some(false);
//                 }
//             }
//         }

//         if end_index <= start_index {
//             return None;
//         }

//         let new_info = ColumnInfo { info: new_info };

//         Some((start_index..end_index, new_info))
//     }

//     pub fn try_paint(&mut self, info: &ColumnInfo) {
//         let Some((range, new_info)) = self.simplify(info, 0..self.cells.len()) else {
//             return;
//         };

//         self.map_on_range(range.clone(), |column| {
//             column.paint_by_info(&new_info);
//         });

//         let Some((range, new_info)) = self.simplify(&new_info, range) else {
//             return;
//         };

//         for i in range.clone() {
//             if let Some(value) = self.cells[i] {
//                 if value {
//                 } else {
//                     if new_info.info[0] > i {
//                         for j in range.start..i {
//                             self.cells[j] = Some(false);
//                         }
//                         self.map_on_range(i + 1..range.end, |column| {
//                             column.try_paint(&new_info);
//                         });
//                         return;
//                     }
//                 }
//                 break;
//             }
//         }

//         for i in range.clone().rev() {
//             if self.cells[i].is_some() {
//                 if self.cells[i] == Some(false) {
//                     if new_info.info[new_info.info.len() - 1] > range.end - i - 1 {
//                         for j in i..range.end {
//                             self.cells[j] = Some(false);
//                         }
//                         self.map_on_range(range.start..i, |column| {
//                             column.try_paint(&new_info);
//                         });
//                         return;
//                     }
//                 }
//                 break;
//             }
//         }
//         self.map_on_range(range, |column| {
//             column.paint_by_info(&new_info);
//         });
//     }
// }

impl Board {
    pub fn try_paint(&mut self) {
        let mut current_hash = self.width * self.height;
        let mut new_hash = 0;

        while current_hash != new_hash {
            current_hash = new_hash;
            for index in 0..self.height {
                let mut row = self.get_row(index);
                row = row.try_fit(&self.infos.rows[index]).unwrap();
                self.set_row(index, row);
            }

            for index in 0..self.width {
                let mut column = self.get_column(index);
                column = column.try_fit(&self.infos.columns[index]).unwrap();
                self.set_column(index, column);
            }
            new_hash = self.cells.iter().filter(|&cell| cell.is_some()).count();
        }
    }

    pub fn painted_rate(&self) -> f32 {
        self.cells.iter().filter(|&cell| cell.is_some()).count() as f32
            / (self.width * self.height) as f32
    }

    pub fn verify(self, painted_board: PaintedBoard) -> bool {
        for (cell, original_cell) in self.cells.iter().zip(painted_board.cells) {
            if cell.is_some_and(|v| v != original_cell) {
                return false;
            };
        }
        return true;
    }
}

impl Column {
    pub fn full(lenght: usize, value: Option<bool>) -> Self {
        Column {
            cells: vec![value; lenght],
        }
    }

    pub fn slice(&self, range: Range<usize>) -> Self {
        Column {
            cells: self.cells[range].iter().cloned().collect(),
        }
    }

    pub fn fit_in(&self, other: &Column) -> bool {
        self.cells
            .iter()
            .zip(other.cells.iter())
            .all(|(a, b)| a.is_none() || b.is_none() || a.unwrap() == b.unwrap())
    }

    pub fn try_paint_using_pd(&mut self, info: &ColumnInfo) {
        self.try_fit(info);

        println!("{:?}({:?})", info.info, self)
    }

    pub fn try_fit(&mut self, info: &ColumnInfo) -> Option<Column> {
        if info.info.len() == 0 {
            Some(Column::full(self.cells.len(), Some(false)))
        } else if info.info.len() == 1 {
            let num = info.info[0];

            if num > self.cells.len() {
                return None;
            }

            let mut final_column = None;

            for i in 0..=self.cells.len() - num {
                let column = Column::full(i, Some(false))
                    + Column::full(num, Some(true))
                    + Column::full(self.cells.len() - num - i, Some(false));
                if column.fit_in(self) {
                    if final_column.is_none() {
                        final_column = Some(column);
                    } else {
                        final_column = Some(final_column.unwrap() | column);
                    }
                }
            }

            final_column
        } else {
            let num = info.info[0];

            let mut final_column = None;

            if self.cells.len() < num {
                return None;
            }

            for i in 0..=self.cells.len() - num {
                if self.cells.len() >= num + 1 + i {
                    let others_column =
                        self.slice(i + num + 1..self.cells.len())
                            .try_fit(&ColumnInfo {
                                info: info.info[1..info.info.len()].iter().cloned().collect(),
                            });

                    if let Some(others_column) = others_column {
                        let column = Column::full(i, Some(false))
                            + Column::full(num, Some(true))
                            + Column::full(1, Some(false))
                            + others_column;
                        if column.fit_in(self) {
                            if final_column.is_none() {
                                final_column = Some(column);
                            } else {
                                final_column = Some(final_column.unwrap() | column);
                            }
                        }
                    }
                }
            }

            final_column
        }
    }
}

impl Add for Column {
    type Output = Column;
    fn add(self, rhs: Self) -> Self::Output {
        let cells = self.cells.iter().chain(rhs.cells.iter()).cloned().collect();
        Column { cells }
    }
}

impl BitOr for Column {
    type Output = Column;
    fn bitor(self, rhs: Self) -> Self::Output {
        Column {
            cells: self
                .cells
                .iter()
                .zip(rhs.cells.iter())
                .map(|(a, b)| match (a, b) {
                    (Some(v1), Some(v2)) if v1 == v2 => Some(*v1),
                    _ => None,
                })
                .collect(),
        }
    }
}
