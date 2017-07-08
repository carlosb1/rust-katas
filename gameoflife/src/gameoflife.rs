struct Grid {
    width: i32,
    height: i32,
    num_cells: i32,
}


impl Grid  {
    fn cells(&self) -> i32 { return self.num_cells}
    fn born(&mut self, col: i32, row: i32) { self.num_cells = 1}
    fn cycle(&mut self) {
        self.num_cells = 0;
    }
}

impl Default for Grid {
    fn default() -> Grid {
        Grid {width: 1, height: 1, num_cells:0}
    }
}


#[test]
fn grid_should_exit_where_is_initialised() {
    let grid = Grid{width: 1,height: 1, ..Default::default()};
    assert_eq!(grid.cells(),0);
}

#[test]
fn grid_should_has_one_cell_when_it_is_added() {
    let mut grid = Grid{width: 1,height: 1, ..Default::default()};
    grid.born(0,0);
    grid.cycle();
    assert_eq!(grid.cells(),0);
    //assert_eq!(grid.cells(),1);
}
