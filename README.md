# RCADA

A SCADA system written in Rust.

## Building

### Linux

```bash
cargo build --workspace

# Release build
cargo build --workspace --release
```
## Running

### Linux

```bash
# Run the server
cargo run -p rcada_server
```

The server starts on `http://127.0.0.1:8080`

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/api/v1/tags` | Create a new tag |
| GET | `/api/v1/tags` | List all tags |
| GET | `/api/v1/tags/{name}` | Get a specific tag |
| PUT | `/api/v1/tags/{name}/value` | Update tag value |
| DELETE | `/api/v1/tags/{name}` | Delete a tag |

### Create Tag Request

```json
{
  "name": "temperature_sensor",
  "unit": "CelsiusDegree",
  "data_type": "Float32"
}
```

### Update Value Request

```json
{
  "value": 25.5,
  "timestamp": "2026-01-01T10:30:00Z"
}
```
