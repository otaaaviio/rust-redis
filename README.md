# Building Redis with rust

* [Building Redis with rust](#building-redis-with-rust)
  * [Introduction](#introduction)
  * [RDB Persistence](#rdb-persistence)
  * [Replication](#replication)
  * [Streams](#streams)
  * [Transactions](#transactions)

## Introduction
- [x] Bind to a port
- [x] Respond to PING
- [x] Respond to multiple PINGs
- [x] Handle concurrent clients
- [x] Implement the ECHO command
- [x] Implement the SET & GET commands
- [x] Expiry

## RDB Persistence
- [ ] RDB file config
- [ ] Read a key
- [ ] Read a string value
- [ ] Read multiple keys
- [ ] Read multiple string values
- [ ] Read value with expiry

## Replication
- [x] Configure listening port
- [x] The INFO command
- [x] The INFO command on a replica
- [x] Initial Replication ID and Offset
- [x] Send handshake (1/3)
- [x] Send handshake (2/3)
- [x] Send handshake (3/3)
- [x] Receive handshake (1/2)
- [x] Receive handshake (2/2)
- [ ] Empty RDB Transfer
- [ ] Single-replica propagation
- [ ] Multi Replica Command Propagation
- [ ] Command Processing
- [ ] ACKs with no commands
- [ ] ACKs with commands
- [ ] WAIT with no replicas
- [ ] WAIT with no commands
- [ ] WAIT with multiple commands

## Streams
- [ ] The TYPE command
- [ ] Create a stream
- [ ] Validating entry IDs
- [ ] Partially auto-generated IDs
- [ ] Fully auto-generated IDs
- [ ] Query entries from stream
- [ ] Query with -
- [ ] Query with +
- [ ] Query single stream using XREAD
- [ ] Query multiple streams using XREAD
- [ ] Blocking reads
- [ ] Blocking reads without timeout
- [ ] Blocking reads using $

## Transactions
- [ ] The INCR command (1/3)
- [ ] The INCR command (2/3)
- [ ] The INCR command (3/3)
- [ ] The MULTI command
- [ ] The EXEC command
- [ ] Empty transaction
- [ ] Queueing commands
- [ ] Executing a transaction
- [ ] The DISCARD command
- [ ] Failures within transactions
- [ ] Multiple transactions

