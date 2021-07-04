use meshvox::Voxels;
use std::fs::File;
use std::io::{BufWriter, Write};
use csv::Writer;
use clap::{Arg, App};

fn get_level(max_val : u32) -> usize {
    let mut max_dim = 1u32;
    for level in (0..20).rev() {
        if max_val <= max_dim {
            return level;
        }

        max_dim <<= 1;
    }

    panic!("Voxel grid is too large: {}", max_val);
}

fn voxels_to_obj(pyramid : &Voxels<f32>) {
    let (v, i) = pyramid.vertices_indices();

    let file = File::create("out.obj").expect("Failed to create output file");
    let mut file = BufWriter::new(file);
    for vertex in v {
        writeln!(
            file,
            "v {:0.6} {:0.6} {:0.6}",
            vertex[0], vertex[1], vertex[2]
        ).expect("Failed to write line to OBJ file");
    }
    for index in i.iter().step_by(3) {
        writeln!(file, "f {} {} {}", index + 1, index + 2, index + 3)
        .expect("Failed to write line to OBJ file");
    }
    file.flush().expect("Failed to flush output file");
}

fn main() {
    let matches = App::new("mesh2sculptr")
        .version("0.1.0")
        .about("OBJ -> SculptrVR Converter")
        .arg(Arg::with_name("file")
                            .help("Sets input OBJ file")
                            .required(true)
                            .index(1)
                            .takes_value(true))
        .arg(Arg::with_name("resolution")
            .long("resolution")
            .help("Sets the resolution of the voxel output (i.e. num voxels per axis)")
            .default_value("100"))
        .arg(Arg::with_name("fill")
            .long("fill")
            .help("Attempt to fill the inside of the mesh with voxels"))
        .arg(Arg::with_name("objviz")
            .long("objviz")
            .help("In addition to the CSV, output an OBJ representing the voxelization (useful for debugging)"))
        .get_matches();


    let obj_file = matches.value_of("file").unwrap();
    let resolution_str = matches.value_of("resolution").unwrap();
    let fill = matches.is_present("fill");
    let objviz = matches.is_present("objviz");

    let resolution : i32 = resolution_str.parse().unwrap();

    let load_options = tobj::LoadOptions {
        triangulate: true,
        ..Default::default()
    };

    let (models, _materials) =
        tobj::load_obj(
            &obj_file,
            &load_options
        )
        .expect("Failed to load OBJ file");

    println!("Gathering mesh data for {} models...", models.len());
    let mut vertices : Vec<[f32; 3]> = vec![];
    let mut indices : Vec<usize> = vec![];
    for (_i, m) in models.iter().enumerate() {
        let mesh = &m.mesh;

        let mesh_vertices : Vec<[f32; 3]> = mesh.positions
        .chunks_exact(3)
        .map(|i| [i[0], i[1], i[2]])
        .collect();

        let vert_index_base = vertices.len();
        let mesh_indices : Vec<usize> = mesh.indices.iter().cloned()
        .map(|i| (i as usize) + vert_index_base).collect();

        vertices.extend(mesh_vertices);
        indices.extend(mesh_indices);
    }

    println!("Scaling model to fit -1..1 bounding box...");
    let max_pos = vertices.iter().cloned().flatten().map(|v| v.abs()).fold(0./0., f32::max);
    let s = 1.0 / max_pos;
    vertices.iter_mut().for_each(|p| *p = [p[0] * s, p[1] * s, p[2] * s]);

    println!("Voxelizing...");
    let box_size : f32 = 2.0 / (resolution as f32);
    let mut voxelization = Voxels::voxelize(&vertices, &indices, box_size);

    if fill {
        println!("Filling gaps...");
        voxelization.fill();
    }

    if objviz {
        println!("Outputting OBJ visualization...");
        voxels_to_obj(&voxelization);
    }

    println!("Writing Data.csv...");
    let (min, max) = voxelization.min_max();
    let max_val = min.iter().chain(max.iter()).map(|x| x.abs() as u32).max()
    .expect("Could not determine voxelization size");
    let voxelization_level = get_level(max_val);
    
    let mut wtr = Writer::from_path("Data.csv").expect("Failed to open output CSV");
    wtr.write_record(&["X", "Y", "Z", "level", "R", "G", "B", "mat"])
    .expect("Failed to write CSV row");

    for p in voxelization.grid_positions.iter() {
        // Unreal Engine (and Sculptr) is Z-up, and we assume our model is Y-up, so swap Y/Z axis
        wtr.serialize(&(p[0], p[2], p[1], voxelization_level, 233, 163, 201, 255))
        .expect("Failed to write CSV row");
    }

    wtr.flush().expect("Failed to flush output CSV");
}
