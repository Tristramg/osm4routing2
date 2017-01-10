extern crate osm4routing;
extern crate docopt;
use docopt::Docopt;

fn main() {

    const USAGE: &'static str = "
Usage: osm4routing <source.osm.pbf>";
    let args = Docopt::new(USAGE).unwrap().parse().unwrap_or_else(|e| e.exit());
    let filename = args.get_str("<source.osm.pbf>");
    match osm4routing::reader::read(filename) {
        Ok((nodes, edges)) => osm4routing::writers::csv_writer(nodes, edges),
        Err(error) => println!("Error: {}", error),
    }
}
