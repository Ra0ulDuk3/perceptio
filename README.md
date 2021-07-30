Perceptio
============


Perceptio is a rust application that utilizes the tokio asynchronous runtime to multi-thread the execution of my favorite enummeration and scanning tools for working on http://hackthebox.eu machines

learn more about the runtime [here](https://tokio.rs/)

perceptio takes in a single argument, the ip address of the target machine, and runs various scans using a number of tools.


list of currently used tools:
- nmap
- gobuster
- nikto


Future improvements:
- take in list of ips to perform enumeration of all of them
- option to perform wpscan
- windows crap
