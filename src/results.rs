use rand::Rng;

use crate::nonogram::{Board, Column, PaintedBoard, PaintedColumn};

pub fn get_mean_rate_painting_columns(
    rng: &mut impl Rng,
    size: usize,
    p: f64,
    q: f64,
    num_test: usize,
) -> Result<f32, ()> {
    let mut sum = 0.0;

    for _ in 0..num_test {
        let painted_column = PaintedColumn::new_random(rng, p, size);
        let info = painted_column.get_info();
        let mut column = Column::new_ramdom_from(&painted_column, rng, q);
        let new_column = column.try_fit(&info).unwrap();

        sum += new_column.painted_rate();

        if !column.verify(painted_column) {
            return Err(());
        }
    }

    Ok(sum / (num_test as f32))
}

pub fn get_mean_rate_painting_nonogram_board(
    rng: &mut impl Rng,
    size: usize,
    p: f64,
    num_test: usize,
) -> Result<f32, ()> {
    let mut sum = 0.0;

    for _ in 0..num_test {
        let painted_board: PaintedBoard = PaintedBoard::new_random(rng, size, size, p);
        let mut board: Board = painted_board.into_empty_board();

        board.try_paint();

        sum += board.painted_rate();

        if !board.verify(painted_board) {
            return Err(());
        }
    }

    Ok(sum / num_test as f32)
}
