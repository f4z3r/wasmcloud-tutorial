# Echo Component

## Purpose

The echo component is a small component exposing an HTTP API. This API allows to:

- create bookings,
- retrieve bookings,
- delete bookings.

On each API call, it pushes an event to an event streaming system in case components are interested
in booking events, and then forwards these requests to another system managing the bookings.

Its API is as follows:

### `GET`: `/<id>`

Retrieve a booking with ID `id`.

### `POST`: `/<id>`

Create a booking with ID `id`. This expects a JSON payload of the following form:

```json
{
  "booking": "<booking text>"
}
```

### `DELETE`: `/<id>`

Delete booking with ID `id`.

## Building

You can build the code using the following command:

```sh
wash build
# when using devbox
devbox run build
```
