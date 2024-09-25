# Payment Microservice gRPC

A high-performance payment microservice implemented in Rust using gRPC.

## Overview

This project implements a payment processing microservice using Rust and gRPC. It provides a scalable and efficient solution for handling payment transactions in a distributed system.

## Features

- gRPC-based API for payment processing
- Secure transaction handling
- Scalable architecture
- High performance due to Rust implementation
- Supports multiple payment methods

## Prerequisites

- Rust (latest stable version)
- Cargo
- Protocol Buffers compiler (protoc)

## Installation

1. Clone the repository:
   ```
   git clone https://github.com/caiotheodoro/payment-microservice-grpc.git
   cd payment-microservice-grpc
   ```

2. Build the project:
   ```
   cargo build --release
   ```

## Usage

1. Start the server:
   ```
   cargo run --bin server
   ```

2. Run the client (for testing):
   ```
   cargo run --bin client
   ```
