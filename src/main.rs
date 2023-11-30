mod load_mesh;
mod mesh;
mod save_mesh;

fn main() {
    // Small example
    let mesh1 = load_mesh::from_obj("src/in/cow.obj");
    save_mesh::to_binary("src/out/cow.dat", &mesh1);
    let mesh2 = load_mesh::from_binary("src/out/cow.dat");
    save_mesh::to_obj("src/out/cow.obj", &mesh2); // this exactly matches the original obj

    // Large example
    let mesh3 = load_mesh::from_obj("src/in/angel.obj"); //
    save_mesh::to_binary("src/out/angel.cbm", &mesh3);
    let mesh4 = load_mesh::from_binary("src/out/angel.cbm");
    save_mesh::to_obj("src/out/angel.obj", &mesh4);
}
