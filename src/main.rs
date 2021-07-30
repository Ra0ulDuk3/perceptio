// step zero: collect command line arguments
// setp zero.3: create useful help message 
// step zero.5: create outline of all commands that will be run 
// step one: create multi threaded environment
// step two: create output dirs
// step three: create task spawner
// step four: test task spawner, and output dir creation
// step five: create tasks for each scan
// step six: callbacks for each  task
// step seven: hook up task spawner with each task
//

use std::env;
use tokio::task;
use std::process::Command;

#[tokio::main]
async fn main() {
    println!("                                |  _)       
 __ \\   _ \\  __| __|  _ \\ __ \\  __| |  _ \\  
 |   |  __/ |   (     __/ |   | |   | (   | 
 .__/ \\___|_|  \\___|\\___| .__/ \\__|_|\\___/  
_|                       _|                 \n\n\n\n");


    // parse command line arguments into a vec of strings
    let args: Vec<String> = env::args().collect();
    
    // make sure we only have one argument
    assert_eq!(args.len(), 2, "[ERROR]: invalid number of arguments\nsee \n\n$ perceptio help )");

    // print help message or start scanning with the ip
    match &*args[1] {
        "help" => println!("usage: perceptio <target ipv4 address>"),
        
        _ => {
            // assuming ip is in correct format

            let ip = &args[1];
            // create clones of the ip for tasks
            let nmap_ip = ip.clone();
            let gobuster_ip = ip.clone();
            let nikto_ip = ip.clone();

            // create directories
            let cmd = "mkdir"; 
            Command::new(cmd).args (&[ 
                   "nmap",
                   "gobuster",
                   "nikto",
                    ])
            .output()
            .expect("Failed to create directories");

            // spawn tasks
            let nmap_handle = task::spawn(async move {
                nmap(nmap_ip);
            });
            let gobuster_handle = task::spawn(async move {
                gobuster(gobuster_ip);
            });
            let nikto_handle = task::spawn(async move {
                nikto(nikto_ip);
            });

            let nmap_out = nmap_handle.await.unwrap();
            println!("{:?}", nmap_out);
            let gobuster_out = gobuster_handle.await.unwrap();
            println!("{:?}", gobuster_out);
            let nikto_out = nikto_handle.await.unwrap();
            println!("{:?}", nikto_out);

        },
    }
}

fn nmap(ip: String) { 
    let cmd = "sudo nmap"; 
    let output = Command::new(cmd).args (&[ 
           "-sC",
           "-sV",
           "--traceroute",
           "-p-",
           "-O",
           "-vv",
           "-oA",
           "nmap/initial-scan",
           &ip
            ])
    .output()
    .expect("Failed to execute Nmap");

    println!("{:?}", output); 
    
}

fn gobuster(ip: String) { 
    let cmd = "gobuster"; 
    let domain = format!("http://{:}/", &ip);

    let output = Command::new(cmd).args (&[ 
           "-fw",
           "-o",
           "gobuster/initial-scan",
           "-q",
           "-u",
           &domain,
           "-w",
           "/opt/SecLists/Discovery/DNS/subdomains-top1million110000.txt"
            ])
    .output()
    .expect("Failed to execute gobuster");

    println!("{:?}", output); 
    
}
fn nikto(ip: String) { 
    let cmd = "nikto"; 
    let output = Command::new(cmd).args (&[ 
           "-host",
           &ip,
           "-Plugins",
           "-o",
           "nikto/initial-scan",
            ])
    .output()
    .expect("Failed to execute nikto");

    println!("{:?}", output); 
    
}
