<h1>
<br>
<br>
ThetaData options data firehose client.
<br>
<br>
</h1>
A cross platform websockets client that collects options trade, quote and ohlc transactions/messages and saves them locally.
<hr>
DISCLAIMER: This is a personal project and is not affiliated with ThetaData in any way. I am not a programmer and this is my first attempt at programming anything in Rust. This project is not intended for production use. It is a learning project and should be used for educational purposes only. 
<hr>

### Features:

now when it runs, it shows real time stats like this:
```
❯ ./target/release/playground
Connected to: ws://127.0.0.1:25520/v1/events
Received messages: 101718 [~ 959.60 / sec. ≡ ~ 101718.00 / min.] [~ delay: 2.80 sec.]
```
!!! also, it maps trade with quote and ohlc records. the aggregate feed is saved in agg.csv. !!!




<hr>

### Compile
Simply run the following command on mac/windows/linux:

```
git clone https://github.com/dcrash9/tdfirehose.git
cd tdfirehose
```

```
cargo build
```
the build will be in the `target/debug` folder.


### Run
```
cargo run

or

./target/debug/tdfirehose
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
-u, --url <URL>    Default: 'ws://127.0.0.1:25520/v1/events'
-V, --version      Print version information
```

the default url is `ws://127.0.0.1:25520/v1/events`. you can change it at start like this:
```
./tdfirehose -u ws://10.0.0.5:8080/v1/events
```

### Exit
To exit just press `Ctrl + C`



<hr>

### The saved files are in the following format:

trade.csv
```
timestamp,root,dte,expiration,strike,right,symbol,size,price,exchange,sequence,condition,ms_of_day,date
2024-04-30 11:27:01.290,SPY,2,20240502,508000,C,SPY240502C00508000,1,2.72,6,1892502965,125,41221290,20240430
2024-04-30 11:27:01.301,BLK,17,20240517,800000,C,BLK240517C00800000,3,2.05,43,-170898468,131,41221301,20240430
2024-04-30 11:27:01.328,CGC,3,20240503,8500,P,CGC240503P00008500,5,0.7,1,-1448905863,18,41221328,20240430
```

quote.csv
```
timestamp,root,dte,expiration,strike,right,symbol,bid_condition,bid_exchange,bid_size,bid,ask,ask_size,ask_exchange,ask_condition,ms_of_day,date
2024-04-30 11:26:47.731,SPY,2,20240502,508000,C,SPY240502C00508000,50,65,6,2.73,2.74,45,1,50,41207731,20240430
2024-04-30 11:27:01.286,SPXW,0,20240430,5035000,P,SPXW240430P05035000,50,5,350,0.55,0.6,191,5,50,41221286,20240430
2024-04-30 10:21:16.143,BLK,17,20240517,800000,C,BLK240517C00800000,50,9,23,1.95,2.3,13,9,50,37276143,20240430
```

ohlc.csv
```
timestamp,root,dte,expiration,strike,right,symbol,open,high,low,close,volume,count,ms_of_day,date
2024-04-30 11:27:01.290,SPY,2,20240502,508000,C,SPY240502C00508000,3.38,4.01,2.55,2.72,2577,390,41221290,20240430
2024-04-30 11:27:01.301,BLK,17,20240517,800000,C,BLK240517C00800000,2.15,2.15,2.05,2.05,5,3,41221301,20240430
2024-04-30 11:27:01.328,CGC,3,20240503,8500,P,CGC240503P00008500,0.67,0.79,0.5,0.7,517,127,41221328,20240430
```

<hr>

### Roadmap
- [ ] use config file for wider configuration and server alternatives.
- [X] (Yes BUT Unoptimised) map TRADE with QUOTE and OHLC records.
- [X] (Yes BUT Unoptimised) normalize & clean data.
- [x] (Partial) realtime stats & analytics.
- [ ] save full trade record to parquet (duckdb, arctic, etc.)


<hr>

[ThetaData](https://www.thetadata.net) rust client for US Option - [Full Trade Stream](https://http-docs.thetadata.us/docs/theta-data-rest-api-v2/3ikwxqsz6m60m-full-trade-stream).