<h1>
<br>
<br>
ThetaData options data firehose client.
<br>
<br>
</h1>
A cross platform websockets client that saves trade, quote and ohlc options data in csv files.
<hr>

## To Use
Simply run the following command on mac/windows/linux:

```
git clone https://github.com/dcrash9/tdfirehose.git
cd tdfirehose
```
```
cargo run --release
```

or to install:
```
cargo install tdfirehose
```

Command line options:
```
❯ ./tdfirehose -h
tdfirehose 0.1.0
A cross platform options data client.

USAGE:
tdfirehose [OPTIONS]

OPTIONS:
-h, --help         Print help information
-u, --url <URL>
-V, --version      Print version information
```

the default url is `ws://127.0.0.1:25520/v1/events`. you can change it at start like this:
```
./tdfirehose -u ws://10.0.0.5:8080/v1/events
```

## Exit
To exit just press `Ctrl + C`

<hr>


[ThetaData](https://www.thetadata.net) rust client for US Option - [Full Trade Stream](https://http-docs.thetadata.us/docs/theta-data-rest-api-v2/3ikwxqsz6m60m-full-trade-stream).