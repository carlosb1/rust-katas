struct Grid {
    width: i32,
    height: i32,
    num_cells: i32,
    cells: Vec<i32>
}

impl Grid  {
    fn cells(&self) -> i32 { return self.num_cells}
    fn born(&mut self, col: i32, row: i32) { 
        //TODO refactor this num_cells    
        let posic = (self.width*row) + col;
        self.cells[posic as usize] = 1;
        self.num_cells +=1; 


    }
    fn index(&self, col: i32, row: i32) -> usize {
        return ((row*self.width)+col) as usize;
    }

    fn cycle(&mut self) {
        //TODO NOT WORK!!!
        if (self.cells.len() == 1) {
            self.cells[0] = 0;
            self.num_cells = 0;
            return;
        }

        self.cells[0] = 0;
        self.num_cells = 0;

        let row = 0;
        let col = 0;
        let next_row = (row + 1);
        let prev_row = (row - 1);
        let next_col = (col + 1);
        let prev_col = (col - 1);
        let mut neighbours = Vec::new();
        neighbours.push((prev_col,prev_row));
        neighbours.push((col,prev_row));
        neighbours.push((next_col,prev_row));
        neighbours.push((prev_col,row));
        neighbours.push((next_col,row));
        
        neighbours.push((prev_col,next_row));
        neighbours.push((col,next_row));
        neighbours.push((next_col,next_row));

        let mut num_neighbours = 0;
        for &(neigh_col,neigh_row) in neighbours.iter() {
            if (neigh_col>=0 && neigh_col<self.width && neigh_row>=0 && neigh_row <self.height) {
                let cell_posic = self.index(neigh_col,neigh_row);
                if (self.cells[cell_posic] ==1) {
                     num_neighbours+=1;
                }
            }
        }


        if (num_neighbours >=2 && num_neighbours<=3) {
            self.num_cells=1;
        }
        
    }
    fn new (width: i32, height: i32) -> Grid{
        return Grid {width: width, height: height, num_cells: 0, cells: vec![0;(width * height) as usize]};
    }
}



#[test]
fn grid_should_exit_where_is_initialised() {
    let grid = Grid::new(1,1); 
    assert_eq!(grid.cells(),0);
}

#[test]
fn cell_should_dead_after_one_cycle_in_grid_length_one() {
    let mut grid = Grid::new(1,1); 
    grid.born(0,0);
    grid.cycle();
    assert_eq!(grid.cells(),0);
}
//TODO test multiple cells
/*
   [*][ ]
   [ ][ ]
 */
#[test]
fn cell_should_survive_dead_one_cycle_in_grid_length_two() {
    let mut grid = Grid::new(2,2); 
    grid.born(0,0);
    grid.cycle();
    assert_eq!(grid.cells(),0);
}

/*
   [*][*]
   [*][ ]
 */
#[test]
fn three_cells_live_in_grid_when_a_cycle_then_one_live() {
    let mut grid = Grid::new(2,2); 
    grid.born(0,0);
    grid.born(0,1);
    grid.born(1,0);
    grid.cycle();
    assert_eq!(grid.cells(),1);
}
