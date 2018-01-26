# influxdb-csv-cleaner
Tool to clean CSV content exported from [InfluxDB](https://www.influxdata.com/).

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

## Example

You export data from InfluxDB with this command:

```
influx -database myfarm -precision s -format csv -execute "SELECT temperature FROM condition LIMIT 100"
```

And save to a file _sample.csv_:

```
    name,time,temperature
    condition,1489544029,29.1
    condition,1489544039,29.2
```

Now, you want to remove the first column and convert the time to _Asia/Ho_Chi_Minh_ timezone:

```
influxdb-csv-cleaner sample.csv -t Asia/Ho_Chi_Minh -o clean.csv
```

The ouput will be:

```
    time,temperature
    2017-03-15 09:13:49 +07,29.1
    2017-03-15 09:13:59 +07,29.2
```

Note: The header line can apear many times in the InfluxDB export file, because `influx` client then makes chunked queries to handle big data. But `influxdb-csv-cleaner` tool will skip all of them, except the top line.

You can also use the tool in pipeline to clean on the _stdin_ stream:

```
influx -database myfarm -precision s -format csv -execute "SELECT temperature FROM condition LIMIT 100" | influxdb-csv-cleaner - -t Asia/Ho_Chi_Minh
```
