use std::fs::File;
use std::io::Write;
use std::str;

use crate::mesh::Mesh;

pub mod first_face_serialization {
    pub const U8: u8 = 0b00000000;
    pub const U16: u8 = 0b01000000;
    pub const U32: u8 = 0b10000000;
}

pub mod second_face_serialization {
    pub const I8: u8 = 0b00000000;
    pub const I16: u8 = 0b00010000;
    pub const I32: u8 = 0b00100000;
    pub const U32: u8 = 0b00110000;
}

pub mod third_face_serialization {
    pub const I8: u8 = 0b00000000;
    pub const I16: u8 = 0b00000100;
    pub const I32: u8 = 0b00001000;
    pub const U32: u8 = 0b00001100;
}

pub fn to_binary(filepath: &str, mesh: &Mesh) {
    // dump all positions
    // dump first index of face, then the difference between the first index and the second and third
    let positions = &mesh.positions;
    let faces = &mesh.faces;

    let mut file = File::create(filepath).unwrap();

    // write the number of vertices as u64
    file.write(&(positions.len() as u64).to_le_bytes()).unwrap();

    for point in positions {
        file.write(&point[0].to_le_bytes()).unwrap();
        file.write(&point[1].to_le_bytes()).unwrap();
        file.write(&point[2].to_le_bytes()).unwrap();
    }

    // write the number of faces as u64
    file.write(&(faces.len() as u64).to_le_bytes()).unwrap();
    for face in faces.iter() {
        let ser_type_0: u8; // based on the difference between index
        if face[0] <= u8::MAX as u32 {
            ser_type_0 = first_face_serialization::U8;
        } else if face[0] <= u16::MAX as u32 {
            ser_type_0 = first_face_serialization::U16;
        } else {
            ser_type_0 = first_face_serialization::U32;
        }

        let ser_type_1: u8; // the difference between first face value and second face value
        let diff_1 = face[0] as i64 - face[1] as i64;
        if diff_1 <= i8::MAX as i64 && diff_1 >= i8::MIN as i64 {
            ser_type_1 = second_face_serialization::I8;
        } else if diff_1 <= i16::MAX as i64 && diff_1 >= i16::MIN as i64 {
            ser_type_1 = second_face_serialization::I16;
        } else if diff_1 <= i32::MAX as i64 && diff_1 >= i32::MIN as i64 {
            ser_type_1 = second_face_serialization::I32;
        } else {
            ser_type_1 = second_face_serialization::U32;
        }

        let ser_type_2: u8; // the difference between first face value and third face value
        let diff_2 = face[0] as i64 - face[2] as i64;
        if diff_2 <= i8::MAX as i64 && diff_2 >= i8::MIN as i64 {
            ser_type_2 = third_face_serialization::I8;
        } else if diff_2 <= i16::MAX as i64 && diff_2 >= i16::MIN as i64 {
            ser_type_2 = third_face_serialization::I16;
        } else if diff_2 <= i32::MAX as i64 && diff_2 >= i32::MIN as i64 {
            ser_type_2 = third_face_serialization::I32;
        } else {
            ser_type_2 = third_face_serialization::U32;
        }

        // Write the mini-header byte for each face
        // first 2 bits:  type of the first index integer (u8, u16, u32)
        // second 2 bits: type of the second index integer (i8, i16, i32, u32)
        // third 2 bits: type of the third index integer (i8, i16, i32, u32)
        let all_types: u8 = ser_type_0 | ser_type_1 | ser_type_2;
        file.write(&all_types.to_le_bytes()).unwrap();

        if ser_type_0 == first_face_serialization::U8 {
            file.write(&(face[0] as u8).to_le_bytes()).unwrap();
        } else if ser_type_0 == first_face_serialization::U16 {
            file.write(&(face[0] as u16).to_le_bytes()).unwrap();
        } else if ser_type_0 == first_face_serialization::U32 {
            file.write(&(face[0] as u32).to_le_bytes()).unwrap();
        }

        if ser_type_1 == second_face_serialization::I8 {
            file.write(&(diff_1 as i8).to_le_bytes()).unwrap();
        } else if ser_type_1 == second_face_serialization::I16 {
            file.write(&(diff_1 as i16).to_le_bytes()).unwrap();
        } else if ser_type_1 == second_face_serialization::I32 {
            file.write(&(diff_1 as i32).to_le_bytes()).unwrap();
        } else {
            file.write(&(face[1] as u32).to_le_bytes()).unwrap();
        }

        if ser_type_2 == third_face_serialization::I8 {
            file.write(&(diff_2 as i8).to_le_bytes()).unwrap();
        } else if ser_type_2 == third_face_serialization::I16 {
            file.write(&(diff_2 as i16).to_le_bytes()).unwrap();
        } else if ser_type_2 == third_face_serialization::I32 {
            file.write(&(diff_2 as i32).to_le_bytes()).unwrap();
        } else {
            file.write(&(face[2] as u32).to_le_bytes()).unwrap();
        }
    }
}

pub fn to_obj(filepath: &str, mesh: &Mesh) {
    let positions = &mesh.positions;
    let faces = &mesh.faces;

    let mut file = File::create(filepath).unwrap();

    // write the vertices
    for point in positions {
        write!(file, "v").unwrap();
        for vertex in point {
            write!(file, " {}", vertex).unwrap();
        }
        write!(file, "\n").unwrap();
    }

    // write the faces
    for face in faces {
        write!(file, "f").unwrap();
        for idx in face {
            write!(file, " {}", idx).unwrap();
        }
        write!(file, "\n").unwrap();
    }
}

pub fn to_raw_binary(filepath: &str, mesh: &Mesh) {
    // dump all positions
    // dump first index of face, then the difference between the first index and the second and third
    let positions = &mesh.positions;
    let faces = &mesh.faces;

    let mut file = File::create(filepath).unwrap();

    file.write(&(positions.len() as u64).to_le_bytes()).unwrap();
    for point in positions {
        file.write(&point[0].to_le_bytes()).unwrap();
        file.write(&point[1].to_le_bytes()).unwrap();
        file.write(&point[2].to_le_bytes()).unwrap();
    }

    file.write(&(faces.len() as u64).to_le_bytes()).unwrap();
    for face in faces.iter() {
        file.write(&(face[0] as u32).to_le_bytes()).unwrap();
        file.write(&(face[1] as u32).to_le_bytes()).unwrap();
        file.write(&(face[2] as u32).to_le_bytes()).unwrap();
    }
}
