# command-not-found
command-not-found in rust

## Why?

Mostly to get acquainted with rust and to also look at the speed benefits, here's a not so exhaustive comparison:

```
sergiusens@mirkwood:~/source/command-not-found$ time /usr/lib/command-not-found add-apt-repository
Command 'add-apt-repository' is available in '/usr/bin/add-apt-repository'
add-apt-repository: command not found

real    0m0,144s
user    0m0,136s
sys     0m0,004s
sergiusens@mirkwood:~/source/command-not-found$ time ./target/debug/command-not-found add-apt-repository
The program 'add-apt-repository' is currently not installed. You can install it by typing:
sudo apt install software-properties-common

real    0m0,012s
user    0m0,004s
sys     0m0,004s
sergiusens@mirkwood:~/source/command-not-found$ time /usr/lib/command-not-found ncdu
The program 'ncdu' is currently not installed. You can install it by typing:
sudo apt install ncdu

real    0m0,166s
user    0m0,096s
sys     0m0,032s
sergiusens@mirkwood:~/source/command-not-found$ time ./target/debug/command-not-found ncdu
The program 'ncdu' is currently not installed. You can install it by typing:
sudo apt install ncdu

real    0m0,013s
user    0m0,008s
sys     0m0,000s
```
