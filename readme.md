# JSON schema validator

This experimental project provides a secure JSON schema validation in browser environment. It uses _jsonschema_ Rust package wrapped into a WASM module and exposing functionality within SchemaValidator object.

## Usage

Copy the following file into your output:

| Parameter | File                                                             |
|-----------|------------------------------------------------------------------|
| wasmURL   | @lsolova/json-schema-validator/dist/assets/schema_validator.wasm |

First it must be initialized by passing the deployed WASM file URL.

Then a simple validate can be used, by passing the URL of the schema file (http:// or https:// protocols are accepted) and the data to be validated. If a schema is downloaded, then it is cached until the session exists.

```ts
import { SchemaValidator } from "@lsolova/json-schema-validator";

async function initValidation {
    // Set the wasmURL to the URL of the exposed WASM file (see more details below)
    await SchemaValidator.init(wasmURL);
};

async function validate(schemaURL, data) {
    await SchemaValidator.validate(schemaURL, data);
};
```

This validator supports HTTP/HTTPS references within the schema files (see `$ref`).

```json
{
    "$schema": "https://json-schema.org/draft/2020-12/schema",
    "type": "object",
    "properties": {
        "name": {"type": "string"},
        "email": {"$ref": "http://example.com/schemas/email.schema.json"}
    },
    "required": ["name", "email"]
}
```

### How to add WASM file to your deployment?

#### Using esbuild

TBD

#### Using Vite

Please, use the [explicit URL import](https://vite.dev/guide/assets#explicit-url-imports) feature of Vite passing WASM URL to the _SchemaValidator.init_ function. Everything else will be managed by Vite build.

```ts
import { SchemaValidator } from "@lsolova/json-schema-validator";
import wasmURL from "@lsolova/json-schema-validator/dist/assets/schema_validator.wasm?url";

SchemaValidator.init(wasmURL);
```

## Development

1. Change to _wasm_ directory `cd wasm`
2. Run `cargo install`
3. Run `./build.sh`
4. Change back to root directory `cd ..`
5. Run `npm i`
6. Run `npm run build:worker`

### Local testing

To start the application on localhost, run `npm run serve`.
