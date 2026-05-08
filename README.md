# sunrise

Sunrise is a simple CLI tool to determine the sunrise and sunset times for a given location.

## Usage

```shell
sunrise [location]
```

`location` can be:

- latitude and longitude prefixed with an `@` character
- `.` to fetch the location from the network using [Kamero's Geolocation API](https://geo.kamero.ai/)
- location name prefixed with a `#` character, fetched from [OpenStreetMap](http://nominatim.openstreetmap.org/)
