# Project Setup

## Requirements

- [Docker](https://www.docker.com/get-started)
- [k6](https://k6.io/)

## How to Run

1. Start the services with Docker Compose:

   ```sh
   docker compose up --build
   ```

2. In another terminal, run the k6 tests:
   ```sh
   k6 run src/test.js
   ```
