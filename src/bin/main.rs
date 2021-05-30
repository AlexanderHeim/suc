use suc::sucfile::SucFile;

fn main() {
    let mut sf = SucFile::open("test.suc").unwrap();
    sf.add("Alexander", "4312541").unwrap();
    //sf.remove("Alexander").unwrap();
    println!("{}", sf.check("Alexander", "4312541").unwrap());
}