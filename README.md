## elebuild

### What do you need?

1. Docker
2. `docker-compose`
3. (If compiling) [A rust build environment](https://rustup.rs/)


### Getting it up

Follow the steps below in the given order.
#### Getting the db on

1. Set the env variables Look at [the environment variables section](#environment-variables); Place it in a `.env` file.
2. Use docker compose to start the database
    ```sh
    docker-compose up -d
    ```


#### Compiling it (If you dont have an executable)

1. Use the rust toolchain, preferably the latest stable  one.
2. Build it
   ```sh
   cargo build --release
   ```
3. Your executable will be present in the `target/release` directory.


Note:
1. Use `cargo run --release` to directly compile and run
2. Database needs to be reachable while compiling
#### Running it

1. Run the executable
2. ???
3. Profit

Note: The executable is silent, use the [rest api](api.rest) to talk to it. This file can be interacted with the VSCode extension "Rest Client" (humao.rest-client) for testing, but otherwise these are curl requests that can be used appropriately.


### Environment variables

| VAR               | DESC           | FOR    |
| ----------------- | -------------- | ------ |
| POSTGRES_USER     |                | db     |
| POSTGRES_PASSWORD |                | db     |
| DATABASE_URL      | connection url | server |

### What it has

User -> Name, USN, PWD, email, Dept
Sub -> CC, Sem, Dept, isGlobal, maxCapacity
Booking -> USN,CC

### What it does

1. Mass import users
2. Mass import subjects
3. Users book subjects? Better just add any amount
4. Users can see stats?
5. Report comes with result

### TODO

- [X] Understand existing codebase
- [ ] Consider conventional composition for the sake of my sanity