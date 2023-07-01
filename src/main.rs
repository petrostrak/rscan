use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// address
    #[arg(short, long)]
    addr: String,
    /// rscan -a address -s 443
    #[arg(short, long)]
    single: Option<u16>,
    /// rscan -a address -r 80-443
    #[arg(short, long)]
    range: Option<String>,
    /// rscan -a address -l 80,443, 1080,445
    #[arg(short, long)]
    list: Option<String>,
}

fn main() {
    let args = Cli::parse();

    if let Some(single) = args.single {
        scan_ports(&args.addr, vec![single])
    }

    if let Some(range) = args.range {
        let mut split = range.split("-");
        let start: u16 = match split.next() {
            Some(v) => v.parse().unwrap(),
            None => panic!("range value is not valid, see rscan --help for info"),
        };
        let end: u16 = match split.next() {
            Some(v) => v.parse().unwrap(),
            None => panic!("range value is not valid, see rscan --help for info"),
        };
        let mut list = vec![];
        (start..end).into_iter().for_each(|x| list.push(x));
        scan_ports(&args.addr, list);
    }
}

fn scan_ports(addr: &String, list: Vec<u16>) {}
