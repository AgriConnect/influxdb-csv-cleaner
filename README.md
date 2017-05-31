# influxdb-csv-cleaner
Tool to clean CSV content exported from [InfluxDB](https://www.influxdata.com/).

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
    2017-03-15 09:13:49 +07,29.1,5
    2017-03-15 09:13:59 +07,29.2,5
```

You can also use the tool in pipeline to clean on the _stdin_ stream:

```
influx -database myfarm -precision s -format csv -execute "SELECT temperature FROM condition LIMIT 100" | influxdb-csv-cleaner - -t Asia/Ho_Chi_Minh
```
