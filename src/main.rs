use std::io;

struct Board {
    players: [u64; 2],
}

impl Board {
    fn new(one: u64, two: u64) -> Self {
        Board {
            players: [one, two],
        }
    }

    fn print(&self) {
        for row in (0..6).rev() {
            for col in 0..7 {
                let position = 1 << (col * 6 + row);

                if self.players[0] & position != 0 {
                    print!("R ");
                } else if self.players[1] & position != 0 {
                    print!("Y ");
                } else {
                    print!(". ");
                }
            }

            println!();
        }
    }

    fn moves(&self) -> Vec<u64> {
        let mut moves = Vec::new();
        let all = self.players[0] | self.players[1];

        for col in 0..7 {
            if all & (1 << (col * 6 + 5)) == 0 {
                moves.push(col);
            }
        }

        moves
    }

    fn turn(&self) -> usize {
        ((self.players[0] | self.players[1]).count_ones() % 2) as usize
    }

    fn piece(&mut self, col: u64) -> Result<(), &str> {
        if col > 6 {
            return Err("Column out of range");
        }

        if !self.moves().contains(&col) {
            return Err("Column full");
        }

        for row in 0..6 {
            let mask = 1 << (col * 6 + row);

            if (self.players[0] | self.players[1]) & mask == 0 {
                self.players[self.turn()] |= mask;
                break;
            }
        }

        Ok(())
    }
}

fn main() {
    let mut board = Board::new(0, 0);

    loop {
        board.print();

        let mut col = String::new();
        io::stdin()
            .read_line(&mut col)
            .expect("Failed to read line");

        let col = match col.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Failed to parse input");
                continue;
            }
        };

        match board.piece(col) {
            Ok(_) => (),
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };
    }
}
