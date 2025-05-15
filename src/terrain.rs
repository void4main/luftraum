pub struct SrtmTerrain {
    pub num_rows: usize,
    pub num_cols: usize,
    pub xll_corner: f32,          // x lower left corner (longitude)
    pub yll_corner: f32,          // y lower left corner (latitude)
    pub cell_size: f32,             // center to center
    pub no_data_value: f32,         // -9999 or other
    pub terrain_data: Vec<f32>,     // average heights
}
