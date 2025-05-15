use crate::terrain::SrtmTerrain;

pub fn import_srtm(res: usize) -> SrtmTerrain {
    let data = include_str!(".././assets/srtm_38_02.asc").to_string(); // Elbe, Hamburg
    // let data = include_str!(".././assets/srtm_64_05.asc").to_string(); // Fuji
    let mut data_lines = data.lines();

    let num_cols = data_lines
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .map_or(0, |s| s.parse().unwrap());
    let num_rows = data_lines
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .map_or(0, |s| s.parse().unwrap());

    let xll_corner: f32 = data_lines
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .map_or(0.0, |s| s.parse().unwrap());
    let yll_corner: f32 = data_lines
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .map_or(0.0, |s| s.parse().unwrap());

    let cell_size: f32 = data_lines
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .map_or(0.0, |s| s.parse().unwrap());
    let no_data_value: f32 = data_lines
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .map_or(0.0, |s| s.parse().unwrap());

    let mut data_vec: Vec<f32> = vec![];
    for line in data_lines.enumerate() {
        for date in line.1.split(' ').enumerate() {
            let mut height = date.1.trim().parse::<f32>().unwrap_or(0.0);
            if height == no_data_value {
                height = 0.0;
            }
            if date.0 >= res {
                break;
            }
            data_vec.push(height);
        }
    }

    SrtmTerrain {
        num_rows,
        num_cols,
        xll_corner,
        yll_corner,
        cell_size,
        no_data_value,
        terrain_data: data_vec,
    }
}
