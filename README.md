# influxdb-csv-cleaner

Tool to clean CSV content exported from [InfluxDB](https://www.influxdata.com/).

This tool used to support converting timezone for exported data, but has been dropped in v0.2.0.

## What it does

 - Remove repeated header lines, appearing in the middle of file.
 - Remove first column, which is just table (measurement) name.
 - Convert timestamp in "time" column to a readable format, like "2018-01-30 00:00:00".

## Build

1. For BeagleBone

    - Install _gcc-6-arm-linux-gnueabihf_, _binutils-arm-linux-gnueabihf_ packages.
    - Install _armv7-unknown-linux-gnueabihf_ target for Rust:

    ```sh
    rustup target install armv7-unknown-linux-gnueabihf
    ```

    - Build with command:

    ```sh
    cargo build --target=armv7-unknown-linux-gnueabihf --release
    ```

    - Strip the compiled file to reduce file size:

    ```sh
    /usr/arm-linux-gnueabihf/bin/strip target/armv7-unknown-linux-gnueabihf/release/influxdb-csv-cleaner
    ```

2. For PC

Too simple to tell.

## Example

You export data from InfluxDB with this command:

```sh
influx -database myfarm -precision s -format csv -execute "SELECT temperature FROM condition LIMIT 100 TZ('Asia/Ho_Chi_Minh')"
```

And save to a file _sample.csv_:

```csv
    name,time,temperature
    condition,1489544029,29.1
    condition,1489544039,29.2
```

Now, you want to remove the first column:

```sh
influxdb-csv-cleaner sample.csv -o clean.csv
```

The ouput will be:

```csv
    time,temperature
    2017-03-15 09:13:49,29.1
    2017-03-15 09:13:59,29.2
```

The timestamp column is always convert to readable format. It also help avoid confusing when you use this CSV with graphing tool.

Note: The header line can apear many times in the InfluxDB export file, because `influx` client then makes chunked queries to handle big data. But `influxdb-csv-cleaner` tool will skip all of them, except the top line.

You can also use the tool in pipeline to clean on the _stdin_ stream:

```sh
influx -database myfarm -precision s -format csv -execute "SELECT temperature FROM condition LIMIT 100 TZ('Asia/Ho_Chi_Minh')" | influxdb-csv-cleaner -
```

Please run

```sh
influxdb-csv-cleaner -h
```

to see more other usages.
