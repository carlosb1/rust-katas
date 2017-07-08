struct Grid {
    width: i32,
    height: i32,
}


impl Grid  {
    fn cells(&self) -> i32 { return 0}
}


#[test]
fn grid_should_exit_where_is_initialised() {
    let grid = Grid{width: 1,height: 1};
    assert_eq!(grid.cells(),0);
}

