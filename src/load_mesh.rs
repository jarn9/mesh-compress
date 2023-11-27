use crate::mesh::Mesh;
use crate::save_mesh::first_face_serialization;
use crate::save_mesh::second_face_serialization;
use crate::save_mesh::third_face_serialization;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::str;

pub fn from_obj(filepath: &str) -> Mesh {
    let file: File = File::open(filepath).unwrap();
    let reader: BufReader<File> = BufReader::new(file);

    let mut mesh = Mesh {
        positions: Vec::<[f32; 3]>::new(),
        faces: Vec::<[u32; 3]>::new(),
    };

    let positions = &mut mesh.positions;
    let faces = &mut mesh.faces;

    for line in reader.lines() {
        let line = line.unwrap();
        let line_elements: Vec<&str> = line.split(" ").collect();

        if line_elements[0] == "#" {
            continue; // commment line
        } else if line_elements[0] == "f" {
            if line_elements.len() != 4 {
                panic!("Faces must be triangular")
            }

            let mut new_face = [0; 3];
            new_face[0] = line_elements[1].parse::<u32>().unwrap();
            new_face[1] = line_elements[2].parse::<u32>().unwrap();
            new_face[2] = line_elements[3].parse::<u32>().unwrap();

            faces.push(new_face);
        } else if line_elements[0] == "v" {
            positions.push([
                line_elements[1].parse::<f32>().unwrap(),
                line_elements[2].parse::<f32>().unwrap(),
                line_elements[3].parse::<f32>().unwrap(),
            ]);
        }
    }

    return mesh;
}

pub fn from_binary(filepath: &str) -> Mesh {
    let file = File::open(filepath).unwrap();
    let mut reader = BufReader::new(file);

    let mut mesh = Mesh {
        positions: Vec::<[f32; 3]>::new(),
        faces: Vec::<[u32; 3]>::new(),
    };

    let positions = &mut mesh.positions;
    let faces = &mut mesh.faces;

    // Read in the number of vertices as u64
    let mut position_len_buffer = [0 as u8; 8];
    reader.read_exact(&mut position_len_buffer).unwrap();
    let n_vertices = u64::from_le_bytes(position_len_buffer);

    for _ in 0..n_vertices {
        let mut new_vertex = [0 as f32; 3];
        for coord in 0..3 {
            let mut coord_buf = [0 as u8; 4];
            reader.read_exact(&mut coord_buf).unwrap();
            new_vertex[coord] = f32::from_le_bytes(coord_buf);
        }
        positions.push(new_vertex);
    }

    // Read in the number of vertices as u64
    let mut faces_len_buffer = [0 as u8; 8];
    reader.read_exact(&mut faces_len_buffer).unwrap();
    let n_faces = u64::from_le_bytes(faces_len_buffer);

    for _ in 0..n_faces {
        // read in the face header
        let mut face_type_buffer = [0 as u8; 1];
        reader.read_exact(&mut face_type_buffer).unwrap();
        let face_type_bytes = face_type_buffer[0];

        let mut new_face = [0 as u32; 3];
        if 0b11000000 & face_type_bytes == first_face_serialization::U8 {
            let mut first_face_buf = [0 as u8; 1];
            reader.read_exact(&mut first_face_buf).unwrap();
            new_face[0] = u8::from_le_bytes(first_face_buf) as u32;
        } else if 0b11000000 & face_type_bytes == first_face_serialization::U16 {
            let mut first_face_buf = [0 as u8; 2];
            reader.read_exact(&mut first_face_buf).unwrap();
            new_face[0] = u16::from_le_bytes(first_face_buf) as u32;
        } else {
            let mut first_face_buf = [0 as u8; 4];
            reader.read_exact(&mut first_face_buf).unwrap();
            new_face[0] = u32::from_le_bytes(first_face_buf) as u32;
        }

        if 0b00110000 & face_type_bytes == second_face_serialization::I8 {
            let mut face_buf = [0 as u8; 1];
            reader.read_exact(&mut face_buf).unwrap();
            new_face[1] = (new_face[0] as i64 - i8::from_le_bytes(face_buf) as i64) as u32;
        } else if 0b00110000 & face_type_bytes == second_face_serialization::I16 {
            let mut face_buf = [0 as u8; 2];
            reader.read_exact(&mut face_buf).unwrap();
            new_face[1] = (new_face[0] as i64 - i16::from_le_bytes(face_buf) as i64) as u32;
        } else if 0b00110000 & face_type_bytes == second_face_serialization::I32 {
            let mut face_buf = [0 as u8; 4];
            reader.read_exact(&mut face_buf).unwrap();
            new_face[1] = (new_face[0] as i64 - i32::from_le_bytes(face_buf) as i64) as u32;
        } else {
            let mut face_buf = [0 as u8; 4];
            reader.read_exact(&mut face_buf).unwrap();
            new_face[1] = u32::from_le_bytes(face_buf) as u32;
        }

        if 0b00001100 & face_type_bytes == third_face_serialization::I8 {
            let mut face_buf = [0 as u8; 1];
            reader.read_exact(&mut face_buf).unwrap();
            new_face[2] = (new_face[0] as i64 - i8::from_le_bytes(face_buf) as i64) as u32;
        } else if 0b00001100 & face_type_bytes == third_face_serialization::I16 {
            let mut face_buf = [0 as u8; 2];
            reader.read_exact(&mut face_buf).unwrap();
            new_face[2] = (new_face[0] as i64 - i16::from_le_bytes(face_buf) as i64) as u32;
        } else if 0b00001100 & face_type_bytes == third_face_serialization::I32 {
            let mut face_buf = [0 as u8; 4];
            reader.read_exact(&mut face_buf).unwrap();
            new_face[2] = (new_face[0] as i64 - i32::from_le_bytes(face_buf) as i64) as u32;
        } else {
            let mut face_buf = [0 as u8; 4];
            reader.read_exact(&mut face_buf).unwrap();
            new_face[2] = u32::from_le_bytes(face_buf) as u32;
        }

        faces.push(new_face);
    }

    return mesh;
}
