<h1 align="center">
<br>
<br>
ThetaData options data firehose client.
<br>
<br>
</h1>
<hr>
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

## Exit
To exit just press `ESC` or `Ctrl + C`

<hr>


[ThetaData](https://www.thetadata.net)
[//]: # ()
[//]: # ([ThetaData]&#40;https://www.thetadata.net&#41; rust client for US Option - [Full Trade Stream]&#40;https://http-docs.thetadata.us/docs/theta-data-rest-api-v2/3ikwxqsz6m60m-full-trade-stream&#41;)

[//]: # ()
[//]: # ()
[//]: # (1. make sure you configure the url in main.rs &#40;line 108&#41; with your own server url.)

[//]: # (2. To compile it, execute: "cargo build --release")

[//]: # (3. you can run it with: "./target/debug/tdfirehose")