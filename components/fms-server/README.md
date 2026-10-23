# Current Limitations

## Vehicles

* The operation uses a time range of [-1y, now] for retrieving vehicle data. This means that no data will be
  included from vehicles that have not reported any data during that period.

## Vehicle Positions

* _datetype_: The server supports the `received` type only. Specifying the `created` type yields a 400 response.

## Vehicle Statuses

* _datetype_: The server supports the `received` type only. Specifying the `created` type yields a 400 response.
* _contentFilter_: The server supports the `SNAPSHOT` content only. Specifying any other type yields a 400 response.
