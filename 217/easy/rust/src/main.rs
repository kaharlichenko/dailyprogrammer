use std::fmt;

fn read_usize(source: &mut Iterator<Item=&str>, name: &str) -> Result<usize, String> {
    source
        .next()
        .ok_or(format!("no {} given", name))
        .and_then(|s| {
            s.parse::<usize>()
             .map_err(|_| format!("{} must be positive integer", name))
        })
}

struct Grid {
    min_pile_size: u32,
    rows: Vec<Vec<u32>>,
}

impl Grid {
    fn new(size: usize, it: &mut Iterator<Item=&str>) -> Result<Grid, String> {
        if size == 0 {
            return Err("size cannot be zero".to_string());
        }
        
        let parse_row = |row: &str| -> Result<Vec<u32>, String> { row
            .split(" ")
            .filter(|cell| !cell.is_empty())
            .map(|cell| { cell
                .parse::<u32>()
                .map_err(|_| "each log pile must be positive integer".to_string())
            })
            .collect::<Result<Vec<u32>, String>>()
            .and_then(|piles| {
                if piles.len() != size {
                    Err(format!("each grid row must be exactly {} piles wide", size))
                } else {
                    Ok(piles)
                }
            })
        };

        let rows = try!(it
            .map(parse_row)
            .collect::<Result<Vec<Vec<u32>>, String>>()
            .and_then(|rows| {
                if rows.len() != size {
                    Err(format!("grid must be exactly {} rows long", size))
                } else {
                    Ok(rows)
                }
            }));

        let &min_pile_size = rows.iter()
            .flat_map(|rows| rows.iter())
            .min().unwrap();

        Ok(Grid {
            min_pile_size: min_pile_size,
            rows: rows,
        })
    }
    
    fn add_logs(&mut self, mut number: usize) {
        while number > 0 {
            let desired_size = self.min_pile_size;
            let piles_to_stock = self.rows
                .iter_mut()
                .flat_map(|row| row.iter_mut())
                .filter(|&& mut pile_size| pile_size == desired_size)
                .take(number);
            
            for pile in piles_to_stock {
                *pile += 1;
                number -= 1;
            }
            self.min_pile_size += 1;
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        // Calculate maximum column widths
        let widths = self.rows
            .iter()
            .map(|row| row
                .iter().enumerate()
                // All the piles but the first one
                // get an extra padding that is used
                // to separate piles
                .map(|(i, &pile)| {
                    let width = format!("{}", pile).len();
                    if i > 0 { width + 1 } else { width }
                })
            )
            .fold(
                // Initial widths set all to zeros
                std::iter::repeat(0)
                    .take(self.rows.len())
                    .collect::<Vec<usize>>(),
                |acc, item| {
                    acc.into_iter().zip(item)
                        .map(|(a, b)| std::cmp::max(a, b))
                        .collect::<Vec<_>>()
                }
            );

        // Print row by row using calculated column widths
        for row in &self.rows {
            for (pile, width) in row.iter().zip(widths.iter()) {
                try!(write!(fmt, "{1:>0$}", width, pile));
            }
            try!(writeln!(fmt, ""));
        }

        Ok(())
    }
}

fn read_input() -> Result<(Grid, usize), String> {
    use std::io::Read;

    let mut input = String::new();

    let mut stdin = std::io::stdin();

    if let Err(err) = stdin.read_to_string(&mut input) {
        return Err(format!("error reading stdin: {}", err));
    }

    let mut source = input.lines();

    let size = try!(read_usize(&mut source, "grid size"));
    let logs = try!(read_usize(&mut source, "logs count"));

    let grid = try!(Grid::new(size, &mut source));

    Ok((grid, logs))
}

fn main() {
    let input = read_input();

    match input {
        Ok((mut grid, logs)) => {
            grid.add_logs(logs);
            print!("{}", grid);
        },
        Err(err) => {
            println!("Invalid input: {}", err);
        },
    }
}
