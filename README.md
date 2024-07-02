# axum_backend_example

An example backend project with axum.

## Setup

1. Create the database

    ```sh
    createdb -U postgres axum_backend_db
    ```

1. Configure database connection string

    Create `.env` file based on `.env.example`.

1. Run the migration

    ```sh
    cargo run --bin up
    ```

## Run

```sh
cargo run
```

## Test

### Hello World

```sh
curl  -X GET \
    'http://localhost:3000/' \
    --header 'Accept: */*'
```

```text
Hello, World!
```

### Add a user

```sh
curl  -X POST \
    'http://localhost:3000/users' \
    --header 'Accept: */*' \
    --header 'Content-Type: application/json' \
    --data-raw '{
    "first_name": "John",
    "last_name": "Doe",
    "username": "john_doe"
}'
```

```json
{"id":"373e68a2-b372-44d3-a634-6b8eb425f32a","first_name":"Jane","last_name":"Doe","username":"jane_doe"}
```

### Get all users

```sh
curl  -X GET \
    'http://localhost:3000/users' \
    --header 'Accept: */*'
```

```json
[{"id":"216a1d8c-1216-4260-a051-d6a18622b9d1","first_name":"John","last_name":"Doe","username":"john_doe"},{"id":"373e68a2-b372-44d3-a634-6b8eb425f32a","first_name":"Jane","last_name":"Doe","username":"jane_doe"}]
```

### Get a user

```sh
$ curl  -X GET \
    'http://localhost:3000/users/216a1d8c-1216-4260-a051-d6a18622b9d1' \
    --header 'Accept: */*'
```

```json
{"id":"216a1d8c-1216-4260-a051-d6a18622b9d1","first_name":"John","last_name":"Doe","username":"john_doe"}
```

Note: we escape a newline character in powershell using a backtick `` ` `` instead of a backslash `\`.
