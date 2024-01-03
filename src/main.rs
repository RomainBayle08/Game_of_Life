
use rand::{Rng, thread_rng};


#[derive(Clone)]
struct Cell{
    is_alive: bool
}

struct plateau{
    height : usize,
    width : usize,
    cells : Vec<Cell>
}

impl plateau {
    fn create_plateau( width: usize, height: usize) -> plateau {
        let mut grid: Vec<Cell> = vec![];
        for i in 0..(width * height) {
            let newCell = Cell { is_alive: rand::thread_rng().gen_bool(0.5) };
            grid.push(newCell);
        }
        let this_pleateau = plateau { width, height, cells: grid };
        return this_pleateau
    }

    fn print_grid(&self) {
        for row in self.cells.chunks(self.width) {
            for cell in row {
                print!("{}", if cell.is_alive { "■" } else { "□" })
            }
            println!()
        }
        println!()
    }

    fn update(&mut self) {
        let mut clone_cells = &mut self.cells.clone();
        for i in 1..self.height{
            for j in 1..self.width{
                let index_current_cell = self.index(i ,j) as usize;
                let alive_neighbors = self.find_alive_neighbors(i,j);
                let current_cell : &Cell = &self.cells[index_current_cell];

                if current_cell.is_alive && (alive_neighbors == 1 || alive_neighbors >3){
                    clone_cells[index_current_cell].is_alive = false ;
                }
                else if !current_cell.is_alive && alive_neighbors == 3 {
                    clone_cells[index_current_cell].is_alive = true;
                }
                else {
                    clone_cells[index_current_cell].is_alive = false ;
                }

            }
        }

        self.cells = clone_cells.clone();
    }

    fn find_alive_neighbors(&self,i : usize,j:usize)->usize{
        let mut count = 0;
        for x in (i-1)..=(i+1){
            for y in (j-1)..=(j+1){
                if x>=0 && y>=0 && (x!=j || y!= i ) && x<self.height && y<self.width{
                    let index_cell = self.index(x,y);
                    if self.cells[index_cell].is_alive
                    {
                        count+=1;
                    }
                }
            }
        }

        count



    }

    fn index(&self, i:usize,j:usize)->usize{
        i*self.width+j
    }
}
fn main() {
    let mut jeu = plateau::create_plateau(10, 10);

    for _ in 1..10{
        jeu.print_grid();
        jeu.update();
        println!("-------------------------")
    }
}
