# Rust Quickstart for Keploy

This is a sample **Rust** application using **Actix-web** and **Postgres** (via **SQLx**) to demonstrate how to use Keploy for API testing.

## Prerequisites

- [Docker](https://docs.docker.com/get-docker/) installed.
- [Keploy CLI](https://keploy.io/docs/server/installation/) installed.

## Setup & Run

### 1. using Docker Compose (Recommended)

This sets up both the Rust application and the Postgres database.

```bash
docker compose up
```

The server will be running at `http://localhost:8080`.

## Keploy Integration

### 1. Record Test Cases

Keploy can record API interactions and generate test cases automatically.

1.  **Start Recording**:
    Run the application via Keploy to capture traffic.
    ```bash
    keploy record -c "docker compose up" --container-name rust-app
    ```

2.  **Generate Traffic**:
    Make some API calls to your application.

    **Create an Item:**
    ```bash
    curl -X POST http://localhost:8080/items \
      -H "Content-Type: application/json" \
      -d '{"name": "Keploy Guide", "description": "A guide to using Keploy with Rust"}'
    ```
    *Expected Output:*
    ```json
    {"id":"<some-uuid>","name":"Keploy Guide","description":"A guide to using Keploy with Rust"}
    ```

    **Get Items:**
    ```bash
    curl http://localhost:8080/items
    ```
    *Expected Output:*
    ```json
    [{"id":"<some-uuid>","name":"Keploy Guide","description":"A guide to using Keploy with Rust"}]
    ```

3.  **Stop Recording**:
    Press `Ctrl+C` to stop the application. You should see a `keploy` directory created with your test cases.

### 2. Replay Test Cases

Validate your application with the recorded test cases.

```bash
keploy test -c "docker compose up" --container-name rust-app --delay 10
```

Keploy will run the recorded requests against your application and compare the responses.

## Sample API Endpoints

-   `POST /items` - Create a new item.
-   `GET /items` - Retrieve all items.
-   `GET /items/{id}` - Retrieve a specific item by ID.
