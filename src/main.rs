
use rand::{Rng, thread_rng};
struct Cell{
    isAlive : bool
}

struct plateau{
    height : usize,
    width : usize,
    cells : Vec<Cell>
}

fn create_plateau(width : usize , height : usize)->plateau{
    
    let mut grid: Vec<Cell> = vec![];
    for i in 0..(width*height){
        let newCell = Cell{isAlive: rand::thread_rng().gen_bool(0.5) };
        grid.push(newCell);
    }
    let this_pleateau = plateau{width,height,cells : grid};
    return this_pleateau

}

fn print_grid(width : usize , height : usize){
    let this_plateau = create_plateau(width,height);
    for row in this_plateau.cells.chunks(this_plateau.width){
        for cell in row{
            print!("{}", if cell.isAlive{ "■" } else { "□" })
        }
        println!()
    }
    println!()
}









fn main() {
    print_grid(10,10);
}
