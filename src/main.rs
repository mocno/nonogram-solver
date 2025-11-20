use std::time::Instant;

use rand::{Rng, SeedableRng, rngs::StdRng};

use crate::nonogram::{Board, Column, PaintedBoard, PaintedColumn};

mod nonogram;

fn get_mean_rate_painting_columns(
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

fn get_mean_rate_painting_nonogram_board(
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

fn main() {
    let mut rng = StdRng::seed_from_u64(9);
    let now = Instant::now();
    match get_mean_rate_painting_columns(&mut rng, 15, 0.5, 0.2, 200) {
        Ok(mean) => println!("Média da completude: {:.2}%", 100.0 * mean),
        Err(_) => println!("Deu errado no testes da média"),
    }
    match get_mean_rate_painting_nonogram_board(&mut rng, 15, 0.5, 200) {
        Ok(mean) => println!("Média da completude: {:.2}%", 100.0 * mean),
        Err(_) => println!("Deu errado no testes da média"),
    }
    let elapsed_time = now.elapsed();
    println!("Time: {:} seconds.", elapsed_time.as_secs());
}
