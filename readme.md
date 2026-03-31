# JSON Schema Validator

A JSON schema validator for browser and Node.js environments. It wraps the _jsonschema_ Rust crate as a WebAssembly module, exposing functionality through the SchemaValidator object.

## Installation

```bash
npm install @lsolova/json-schema-validator
```

### Initialization

You must initialize SchemaValidator first by passing the WASM file to `SchemaValidator.init()`. The initialization is an async operation and should be done once at application startup.

### Validation

`SchemaValidator.validate()` is asynchronous and returns a `Promise<void>`. If validation fails, the promise is rejected with a `SchemaValidationError` object.

```ts
type SchemaValidationError = {
    error: string | object
} | {
    // Key is field name, value is the validation error related to that field
    [key: string]: string
}
```

### Error handling example

```ts
try {
    await SchemaValidator.validate(schema, data);
    console.log("Validation passed");
} catch (error) {
    console.error("Validation failed", error);
}
```

### Browser

Browser integration requires the WASM file to be served from your deployment. The WASM file must be copied to your output directory during the build process (see deployment section below).

#### Basic usage

You can validate by passing either:
- **Schema URL**: A URL to a remote schema file (HTTP(S) or a relative path)
- **Schema content**: The schema definition as a JSON object or string

Schemas are automatically cached for the lifetime of the SchemaValidator object, improving validation performance on subsequent calls.

```ts
import { SchemaValidator } from "@lsolova/json-schema-validator";

// Initialize once at app startup
async function initValidation(wasmURL: string) {
    // wasmURL is the path to the exposed WASM file
    await SchemaValidator.init(wasmURL);
    // Optionally pre-register schemas to avoid runtime loading
    await SchemaValidator.registerSchema("my-schema-id", schemaObject);
}

// Use throughout your app
async function validateData(schema: string | object, data: unknown) {
    try {
        await SchemaValidator.validate(schema, data);
        // Validation passed
    } catch (error) {
        // Handle validation errors
    }
}
```

#### Schema references

This validator fully supports JSON Schema `$ref` directives, including HTTP(S) references. Schemas can reference other schemas via URLs or IDs.

```json
{
    "$id": "my-schema-id",
    "$schema": "https://json-schema.org/draft/2020-12/schema",
    "type": "object",
    "properties": {
        "name": {"type": "string"},
        "email": {"$ref": "https://example.com/schemas/email.schema.json"}
    },
    "required": ["name", "email"]
}
```

#### Performance optimization

Validators generated from schemas with a `$id` are internally cached. This can significantly improve performance: subsequent validations with the same validator can be **75% faster** due to caching.

#### Security

Although it doesn't use eval, a special Content Security Policy setting is required in high-security environments.

If a `script-src` directive is set in the Content-Security-Policy header, `'wasm-unsafe-eval'` must be added to it. (See [MDN documentation](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Content-Security-Policy/script-src#unsafe_webassembly_execution) for details.)

```plain
Content-Security-Policy: ...;script-src your entries 'wasm-unsafe-eval'; ...
```

### Node.js

This module runs in Node.js thanks to its native WebAssembly support. It's a pure ESM module and cannot be used with CommonJS.

#### Setup

Load the WASM file once at application startup before performing any validations:

```js
import { readFile } from "node:fs/promises";
import { SchemaValidator } from "@lsolova/json-schema-validator";

// Initialize at app startup
async function initValidator() {
    const wasmBuffer = await readFile(
        "./node_modules/@lsolova/json-schema-validator/dist/assets/schema_validator.wasm"
    );
    await SchemaValidator.init(wasmBuffer);
}

// Use throughout your app
async function validateConfig(data) {
    const schemaContent = await readFile("./schema.json", { encoding: "utf8" });
    try {
        await SchemaValidator.validate(schemaContent, data);
        console.log("Valid");
    } catch (error) {
        console.error("Validation failed:", error);
    }
}
```

#### How it works

Node.js has no window object and this WASM can use _Window.fetch_ only. Therefore in Node environment Reqwest is utilized.

## Deployment

The WASM file must be accessible to your application at runtime. The approach depends on your build tool.

### Vite

Vite has built-in support for assets by URL. Use the [explicit URL import](https://vite.dev/guide/assets#explicit-url-imports) syntax:

```ts
import { SchemaValidator } from "@lsolova/json-schema-validator";
import wasmURL from "@lsolova/json-schema-validator/dist/assets/schema_validator.wasm?url";

async function initValidator() {
    await SchemaValidator.init(wasmURL);
}
```

The WASM file is automatically processed and included in your build output.

### esbuild

TBD

### Node.js

For Node.js, pass the WASM as a Buffer to `init()`. The file location is predictable in `node_modules`:

```js
import { readFile } from "node:fs/promises";
import { SchemaValidator } from "@lsolova/json-schema-validator";
import path from "node:path";

async function initValidator() {
    const wasmPath = path.join(
        process.cwd(),
        "node_modules/@lsolova/json-schema-validator/dist/assets/schema_validator.wasm"
    );
    const wasmBuffer = await readFile(wasmPath);
    await SchemaValidator.init(wasmBuffer);
}
```

## Development

Setup for local development:

```bash
npm install
npm run build
```

### Testing

**Browser**: Start a local web server:
```bash
npm run serve
```

**Node.js**: Run the example:
```bash
npx tsx node-example.ts
```
