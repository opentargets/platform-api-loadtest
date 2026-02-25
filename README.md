# Open Targets Platform API Load Testing

This repository contains code for load testing the Open Targets Platform API. It uses the [goose](https://github.com/tag1consulting/goose) framework to simulate user behavior and measure the performance of the API under load.

## Getting Started
To get started with load testing the Open Targets Platform API, follow these steps:

1. Clone this repository to your local machine.
2. To run the tests against an instance: `cargo run --release -- --host <INSTANCE> --report-file=report.html --no-reset-metrics -t 60 -u 3 -E error.log` 
    1. Play with the parameters to find the best configuration for your needs. For example, you can adjust the duration of the test (`-t`), the number of users (`-u`), and the report file name (`--report-file`).
3. Open the report file (e.g., `report.html`) in a web browser to view the results of the load test.


## Adding New Tests
To add new tests to the load testing suite, follow these steps:
1. In the [queries](./queries) directory, create a new directory for your test (e.g., `my_test`).
2. Inside the new directory, create a `q.gql` file containing the GraphQL query you want to test, a `v.gql` file containing the variables for the query with `$ID` as a placeholder (e.g. [here](./queries/target_page/v.gql)), and an `ids.txt` file containing a newline seperated list of IDs to be used in the variables file.
3. Add the test to the `main.rs` file by adding a new fn that returns a `TransactionResult` where the string passed to the `query` function matches the name of the directory you created in step 1. For example, if you created a directory named `my_test`, your function would look like this:
```rust
async fn my_test(user: &mut GooseUser) -> TransactionResult {
    query(user, "my_test").await
}
```
4. Finally, add the new test function to the `main` function in `main.rs`, for example:
```rust
.register_transaction(transaction!(my_test).set_name("My Test query"))
```
