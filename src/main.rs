mod core;
fn main() {
    let lockfile_content = core::util::lock_file::get_lockfile();
    println!("{}--{}--{}",lockfile_content.as_ref().unwrap().protocol,lockfile_content.as_ref().unwrap().password,lockfile_content.as_ref().unwrap().port);
}