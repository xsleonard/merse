use extra::json;
use extra::serialize::Decodable;
use std::path::Path;
use std::io::fs::File;

#[deriving(Decodable)]
pub struct Config {
    app_name: ~str,
    fullscreen: bool,
    textures: ~[~str],
    width: uint,
    height: uint,
}

pub fn load_config(filename: ~str) -> ~Config {
    // read config from file
    let data = read_config(filename);
    let jsonobj = json::from_str(data);
    let mut decoder = json::Decoder::new(jsonobj.unwrap());
    ~Decodable::decode(&mut decoder)
}

fn read_config(filename: ~str) -> ~str {
    let path = Path::new(filename);
    let on_error = || fail!("open of {:?} failed", path);
    File::open(&path).unwrap_or_else(on_error).read_to_str()
}
