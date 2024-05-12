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
1. now when it runs, it shows real time stats like this:
```
❯ ./target/release/playground
Connected to: ws://127.0.0.1:25520/v1/events
Received messages: 101718 [~ 959.60 / sec. ≡ ~ 101718.00 / min.] [~ delay: 2.80 sec.]
```
2. the application maps trade with quote and ohlc records. the aggregate feed is saved in agg.csv.

3. a log is created which saves every minute a snapshot of the stats.


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

agg.csv
```
timestamp,root,dte,expiration,strike,right,symbol,size,price,exchange,sequence,condition,bid_condition,bid_exchange,bid_size,bid,ask,ask_size,ask_exchange,ask_condition,open,high,low,close,volume,count,ms_of_day,date
2024-04-30 13:39:27.469,SPY,0,20240430,506000,C,SPY240430C00506000,4,0.98,11,2104120248,18,50,69,57,0.98,0.99,314,7,50,2.97,3.9,0.69,0.98,136701,12570,49167469,20240430
2024-04-30 13:43:35.881,TGT,3,20240503,157500,P,TGT240503P00157500,1,0.47,9,322306155,125,50,1,1,0.47,0.5,121,5,50,0.32,0.6,0.32,0.47,279,101,49415881,20240430
2024-04-30 13:43:35.925,AMZN,143,20240920,135000,P,AMZN240920P00135000,1,1.74,7,323080836,18,50,1,1,1.74,1.77,8,47,50,1.7,1.76,1.7,1.74,17,11,49415925,20240430
```


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