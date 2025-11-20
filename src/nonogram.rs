use std::{
    fmt::Debug,
    ops::{Add, Range},
    vec,
};

use rand::Rng;

const BLACK_COLOR: &str = "░";
const WHITE_COLOR: &str = "█";
const UNKNOWN_COLOR: &str = ".";

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

    pub fn get_column_infos(&self) -> ColumnInfos {
        let columns = (0..self.width)
            .map(|i| self.get_column(i).get_info())
            .collect();
        let rows = (0..self.height)
            .map(|j| self.get_row(j).get_info())
            .collect();
        ColumnInfos::new(columns, rows)
    }
}

impl Debug for PaintedBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for k in 0..self.width * self.height {
            if k % self.width == 0 {
                if k != 0 {
                    write!(f, "\n")?;
                }
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

#[derive(Clone)]
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
    fn full(lenght: usize, value: Option<bool>) -> Self {
        Column {
            cells: vec![value; lenght],
        }
    }

    fn slice(&self, range: Range<usize>) -> Self {
        Column {
            cells: self.cells[range].iter().cloned().collect(),
        }
    }

    fn fit_in(&self, other: &Column) -> bool {
        assert_eq!(self.cells.len(), other.cells.len());
        self.cells
            .iter()
            .zip(other.cells.iter())
            .all(|(a, b)| a.is_none() || b.is_none() || a.unwrap() == b.unwrap())
    }

    fn add_info(self, column: &mut Option<Column>) {
        if let Some(column) = column {
            for i in 0..column.cells.len() {
                if self.cells[i] != column.cells[i] {
                    column.cells[i] = None;
                }
            }
        } else {
            *column = Some(self.clone());
        }
    }

    pub fn try_fit(&mut self, info: &ColumnInfo) -> Option<Column> {
        if info.info.len() == 0 {
            return Some(Column::full(self.cells.len(), Some(false)));
        }

        let mut pn: Vec<Option<Column>> = (0..self.cells.len()).map(|_| None).collect();

        let num = info.info[0];
        for j in num - 1..self.cells.len() {
            let mut final_column = None;

            for k in 0..=j + 1 - num {
                let column = Column::full(j + 1 - k - num, Some(false))
                    + Column::full(num, Some(true))
                    + Column::full(k, Some(false));

                if column.fit_in(&self.slice(0..j + 1)) {
                    column.add_info(&mut final_column);
                }
            }

            pn[j] = final_column;
        }

        let mut space = info.info[0] - 1;
        for i in 1..info.info.len() {
            let num = info.info[i];
            space += num + 1;

            for j in (space..self.cells.len()).rev() {
                let mut final_column = None;

                for k in 0..=j - space {
                    let Some(others_column) = pn[j - num - k - 1].clone() else {
                        continue;
                    };
                    let column = others_column
                        + Column::full(1, Some(false))
                        + Column::full(num, Some(true))
                        + Column::full(k, Some(false));

                    if column.fit_in(&self.slice(0..j + 1)) {
                        column.add_info(&mut final_column);
                    }
                }

                pn[j] = final_column;
            }
        }

        pn[self.cells.len() - 1].clone()
    }
}

impl Add for Column {
    type Output = Column;

    fn add(self, rhs: Self) -> Self::Output {
        let cells = [self.cells.as_slice(), rhs.cells.as_slice()].concat();

        Column { cells }
    }
}
