# EngineIdentify API

A high-performance Rust API for identifying game engines based on file structures and filenames.

## Features

- **Fast Detection**: Built with Rust and Axum for high performance.
- **Configurable Rules**: Engine signatures are defined in `engines.json`, allowing for easy updates without recompiling.
- **Weighted Scoring**: Supports weighted signatures to handle ambiguous cases (e.g., folders containing files from multiple engines).
- **Dockerized**: Includes a multi-stage Dockerfile for small image sizes.

## Getting Started

### Prerequisites

- Rust (cargo)
- Docker (optional)

### Running Locally

1.  Clone the repository.
2.  Run the server:
    ```bash
    cargo run
    ```
3.  The server listens on `http://localhost:3000`.

### Running with Docker

```bash
# Build the image
docker build -t engine-identify .

# Run the container
docker run -p 3000:3000 engine-identify
```

## API Usage

### Endpoint: `POST /identify`

Analyzes a list of filenames and returns the most likely game engine.

**Request:**

```json
{
  "files": [
    "Game_Data/Managed/Assembly-CSharp.dll",
    "Game.exe"
  ]
}
```

**Response:**

```json
{
  "engine": "Unity",
  "confidence": 1.0,
  "matches": [
    "Game_Data/Managed/Assembly-CSharp.dll"
  ]
}
```

## Configuration (`engines.json`)

You can add or modify engine signatures in `engines.json`.

```json
[
  {
    "name": "Unity",
    "signatures": [
      { "type": "path_contains", "value": "_data/managed", "weight": 2.0 },
      { "type": "extension", "value": "assets", "weight": 1.5 }
    ]
  }
]
```

### Signature Types
- `path_contains`: Path contains the value (case-insensitive).
- `extension`: File ends with `.<value>`.
- `filename`: Exact filename match.
- `filename_starts_with`: Filename starts with value.
- `filename_ends_with`: Filename ends with value.
- `path_component`: Matches a path component (supports basic wildcards like `level*`).

### Weights
Signatures with higher `weight` contribute more to the final score. Use high weights (>2.0) for unique files that strongly indicate a specific engine.

## License

MIT
