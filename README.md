# Ape Obtainer

This program attempts to download every ape in the Bored Ape Yacht Club collection.

To do this, you'll need an IPFS node. I recommend [IPFS Desktop](https://docs.ipfs.io/install/ipfs-desktop/).

Launch IPFS Desktop, then launch your downloaded program. It will begin downloading apes to a new folder called `apes`, in the same directory as the program.

## Advanced: command line arguments

```
USAGE:
    ape-obtainer.exe [OPTIONS]

FLAGS:
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -h, --host <HOST>            IPFS host name, defaults to 'localhost'
    -l, --location <LOCATION>    Location to download obtained apes to. defaults to './apes'
    -p, --port <PORT>            IPFS port, defaults to 8080
```
