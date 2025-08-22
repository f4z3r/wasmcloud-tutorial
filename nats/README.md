# NATS

<!--toc:start-->
- [NATS](#nats)
  - [Introduction](#introduction)
  - [Running NATS](#running-nats)
  - [Subjects](#subjects)
    - [Wildcards](#wildcards)
  - [Publish Subscribe](#publish-subscribe)
    - [Queues](#queues)
  - [Key Value](#key-value)
  - [Object Storage](#object-storage)
  - [Monitoring](#monitoring)
  - [A Sample Client Application](#a-sample-client-application)
<!--toc:end-->

> This directory provides a DevBox environment. Run `devbox shell` to have all the tools required to
> execute the commands listed here and in subdirectories.

## Introduction

NATS is an event streaming platform which additionally offers key value stores, object stores, and
pub/sub mechanisms. It is built from two main components: NATS core and JetStream. NATS core
provides the base functionality but does not have any persistence. JetStream builds on top of NATS
core to enable persistence.

## Running NATS

We will run NATS with JetStream enabled by executing:

```sh
nats server run --jetstream
```

Generally, NATS would be run on various hosts to provide resilience against outages and better
performance. NATS has self-healing and fault tolerance builtin.

For the commands below, activate the developer context:

```sh
nats context select nats_development
```

## Subjects

NATS performs its routing based on subjects. This is similar to routing keys for RabbitMQ. They are
dot separated strings which provide a tree hierarchy on how messages can be handled. Messages bodies
are simple byte arrays, meaning that we can send any form of data via NATS (similarly to how Kafka
works).

Let us start a small utility which responds to messages sent to a subject:

```sh
nats reply "hello.world" "Hi!"
```

This will reply `"Hi!"` to any request sent to the subject `hello.world`. Thus, when we send a
request to this subject, we should obtain a reply:

```
$ nats req "hello.world" ""
17:09:53 Sending request on "hello.world"
17:09:53 Received with rtt 201.65µs
Hi!
```

### Wildcards

Some commands also take wildcards for subjects. For instance, our reply command can take a wildcard
to reply to messages with different subjects:

```sh
nats reply "hello.*" --command "echo 'Hi {{1}}!'"
```

In this case we are replying to any subjects that have two elements and start with `hello.`. We then
shell out a command to reply whatever matched the wildcard:

```
$ nats req "hello.world" ""
17:18:27 Sending request on "hello.world"
17:18:27 Received with rtt 1.406134ms
Hi world!

$ nats req "hello.f4z3r" ""
17:18:34 Sending request on "hello.f4z3r"
17:18:34 Received with rtt 1.414984ms
Hi f4z3r!

$ # This won't work as it has three parts in the subject and thus won't match.
$ nats req "hello.f4z3r.test" ""
17:19:22 Sending request on "hello.f4z3r.test"
17:19:22 No responders are available
```

## Publish Subscribe

we can also use NATS to subscribe to a suject and receive any messages published to it:

```sh
nats sub "hello.>"
```

Here we use the `>` wildcard which matches any remaining parts. Thus if we now run the following
commands:

```sh
nats pub "hello.world" "Hi!"
nats pub "hello.test" "Hi!"
nats pub "hello.test.f4z3r" "Hi!"
nats pub "test.f4z3r" "Hi!"
```

We should see the following output on the subscriber:

```
17:23:56 Subscribing on hello.>
[#1] Received on "hello.world"
Hi!


[#2] Received on "hello.test"
Hi!


[#3] Received on "hello.test.f4z3r"
Hi!
```

The last message did not match the subject pattern we are subscribing to, and thus did not show in
the received messages.

### Queues

Queues enable to load balance traffic between various instances. This is interesting when one does
not want to to have all subscribers receive all messages, but to spread the messages across
subscribers. This would be typically done on software which scales horizontally and spreads requests
across various instances.

```sh
nats sub "hello.world" --queue greeter
# and in another terminal
nats sub "hello.world" --queue greeter
```

If we now send many requests to this subject, it will be spread accross the queue:

```sh
nats pub "hello.world" "Hi!" --count 1000
```

In my case, it spread the messages 498 and 502 on the other (quite evenly). Note that another
subscriber listening to the same subject without the queue defined will still receive all messages
sent with the subject.

## Key Value

Let us create a bucket:

```sh
nats kv add mybucket
```

And populate it with a key:

```sh
nats kv put mybucket mykey 50
```

Similarly to other messages, keys are subject strings, so we can create another key:

```sh
nats kv create mybucket mykey.test foo
```

> The difference between `put` and `create` is that create ensures that the key did not exist
> before, or has been deleted.

You can of course also retreive keys from the bucket. Check that the two keys differ:

```
$ nats kv get mybucket mykey
mybucket > mykey revision: 1 created @ 22 Aug 25 15:55 UTC

50

$ nats kv get mybucket mykey.test
mybucket > mykey.test revision: 2 created @ 22 Aug 25 15:55 UTC

foo
```

Another powefull tool is to watch. In this case we get notified when a change is performed on a key:

```sh
nats kv watch mybucket mykey
```

To see anything happen, change the value of `mykey`:

```sh
nats kv put mybucket mykey 100
```

## Object Storage

JetStream enables you to store files similar to how the key value store works. Check out the help
message for more information:

```sh
nats object -h
```

## Monitoring

You can monitor the NATS server. For this you need system priviledges though. So change to the
system context:

```sh
nats context select nats_development_system
```

Then you can, for instance, perform a top-like command on the server:

```sh
nats top nats_development
```

## A Sample Client Application

The [`rs-example/`](./rs-example/) directory provides an example publish-subscribe example in Rust
using the NATS server deployed at the very beginning. Run the following command in the `rs-example`
directory to run the example:

```sh
cargo run
```
