# Booking Master Component

## Purpose

The booking master component is responsible for managing bookings. It exposes a
[WIT interface](./wit/world.wit) to manage bookings. In the backend it uses a key/value store to
store the bookings.

## Building

You can build the code using the following command:

```sh
wash build
# when using devbox
devbox run build
```
