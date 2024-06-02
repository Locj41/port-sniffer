
use std::{env, net::{IpAddr, TcpStream}, str::FromStr, process, thread, sync::mpsc, io::{self, Write}, vec};
const MAX:u16 = 65535;
struct Arguments{
    flag:String,
    ipaddr: IpAddr,
    threads: u16,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &str>{
        if args.len() < 2 {
            return Err("Not enough arguments");
        } else if args.len() > 4{
            return Err("Too many arguments");
        }
        let ip = args[1].clone();
        if let Ok(ipaddr) = IpAddr::from_str(&ip) {
                Ok(Arguments { flag: String::from(""), ipaddr, threads: 4})
        } else {
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("-help") && args.len() == 2 {
                println!(
                    "Usage: -j to select how many threads you want\
                    \n       -h or -help to show this help message");
                    Err("help")
            } else if flag.contains("-h") || flag.contains("-help"){
                return Err("Too many arguments");
            } else if flag.contains("-j") {
                let ipaddr = match IpAddr::from_str(&args[3]) {
                    Ok(ipaddr) => ipaddr,
                    Err(_) => return Err("not valid IPADDR; must be IPv4 of IPv6")
                }; 
                let threads = match args[2].parse::<u16>() {
                    Ok(thread) => thread,
                    Err(_) => return Err("Failed to parse number of thread")
                };
                Ok(Arguments { flag, ipaddr, threads })
            }else {
                Err("Invalid syntax")
            }
        }
    }
}

fn main() {
    let args:Vec<String> = env::args().collect();

    let program = args[0].clone();

    let arguments = Arguments::new(&args).unwrap_or_else(|err|{
        if err.contains("help"){
            process::exit(0);
        } else {
            eprintln!("{} problem parsing arguments: {}", program, err);
            process::exit(0);
        }
    });

    let num_threads = arguments.threads;
    let addr = arguments.ipaddr;
    let (tx, rx) = mpsc::channel();
    for i in 0..num_threads {
        let tx = tx.clone();
        thread::spawn(move || {
            scan(tx, i, addr, num_threads);
        });
    }
    let mut out = vec![];
    drop(tx);
    for p in rx {
        out.push(p);
    }
    println!("");
    out.sort();
    for v in out{
        println!("{} is open", v);
    }

}

fn scan(tx:mpsc::Sender<u16>, start_port:u16, addr:IpAddr, num_threads:u16){
    let mut port = start_port + 1;
    while MAX - port > num_threads {       
            match TcpStream::connect((addr, port)) {
                Ok(_) => {
                    print!(".");
                    io::stdout().flush().unwrap();
                    tx.send(port).unwrap();
                },
                Err(_) => {
                }
            };
            port += num_threads;
    }
}

