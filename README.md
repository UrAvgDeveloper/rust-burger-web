CRUD APIs for Burgers using Rust
===============
A spin off for https://punkapi.com/documentation/v2 but for burgers! Using Rust, Diesel, and Rocket Framework. Database used for this POC is Postgres.

## Requirements
* Rust (nightly)
* Docker
* docker-compose

## Usage
```
# Run postgres
docker-compose up -d postgres

# Install diesel
cargo install diesel_cli --no-default-features --features postgres

# Run db migrations
DATABASE_URL=postgres://rocket:rocket@localhost:5432/rocket diesel migration run

# Set Super Admin Creds for the app
source env.sh

# Run the server
cargo run
```

Server will be available at http://localhost:3000/burgers.
# Routes
- `GET` `/burgers` -> Get Burgers

  Query Params:

  | Param | Type |
  | ------------- | ------------- |
  | page (default: 1)  | Number  |
  | per_page (default: 30)| Number  |
  | burger_name (Pattern searching)| String  |

  **Response:**
  ```
  [
    {
        "id": 1,
        "name": "test_burger",
        "image_url": "http://test_burger_image.com",
        "description": "Very gooood burger"
    },
    {
        "id": 2,
        "name": "test_burger_2",
        "image_url": "http://test_burger_image.com",
        "description": "Very gooood burger"
    },
    {
        "id": 3,
        "name": "test_burger_3",
        "image_url": "http://test_burger_image.com",
        "description": "Very gooood burger"
    },
    {
        "id": 4,
        "name": "test_burger_4",
        "image_url": "http://test_burger_image.com",
        "description": "Very gooood burger"
    }
  ]
  ```
- `POST` `/burgers` -> Create Burger

  **Auth:**

  Basic auth, needs username and password of superadmin
  ```
  -H 'authorization: Basic YWRtaW46YWRtaW5AMTIz'
  ```

  **Request Body:**
  ```
  {
    "name": "test_burger_9",
    "image_url": "http://test_burger_image.com",
    "description": "Very gooood burger"
  }
  ```
  **Response:**
  ```
  {
    "id": 6,
    "name": "test_burger_9",
    "image_url": "http://test_burger_image.com",
    "description": "Very gooood burger"
  }
  ```
- `GET` `/burgers/<id>` -> Get a burger by Id

  **Response:**
  ```
  {
    "id": 6,
    "name": "test_burger_9",
    "image_url": "http://test_burger_image.com",
    "description": "Very gooood burger"
  }
  ```

- `GET` `/burgers/random` -> Get a random burger

  **Response:**
  ```
  {
    "id": 6,
    "name": "test_burger_9",
    "image_url": "http://test_burger_image.com",
    "description": "Very gooood burger"
  }
  ```