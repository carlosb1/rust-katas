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

        /*
        let row = 0;
        let col = 0;

        let cell_posic = self.index(col,row);
       
        for index_col in 0..self.height {
            for index_row in 0..self.width {
                let  posic_neight = self.index(index_col,index_row);
                if (self.cells[posic_neight]==0) {
                    self.cells[cell_posic]=0;
                }
            }
        }
        */
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
   [ ][ ]
 */
//#[test]
fn two_cells_should_survive_dead_one_cycle_in_grid_length_two() {
    let mut grid = Grid::new(2,2); 
    grid.born(0,0);
    grid.born(0,1);
    grid.cycle();
    assert_eq!(grid.cells(),0);
}
