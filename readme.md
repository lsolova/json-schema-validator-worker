# JSON schema validator

This is a JSON schema validating solution for browser and Node.js environments. It uses _jsonschema_ Rust package wrapped into a WASM module and exposing functionality in SchemaValidator object.

## Usage

### Browser

Copy the following file into your output directory: `@lsolova/json-schema-validator/dist/assets/schema_validator.wasm`.

SchemaValidator must be initialized first, by passing the deployed WASM file URL.

Then a simple validate call can be used, by passing the content or the URL of the schema file (_http://_, _https://_ and custom _id://_ protocols are supported) and the data to be validated. If an HTTP(S) schema is downloaded once, then it is cached until the SchemaValidator object exists.

```ts
import { SchemaValidator } from "@lsolova/json-schema-validator";

async function initValidation {
    // Set the wasmURL to the URL of the exposed WASM file (see more details below)
    await SchemaValidator.init(wasmURL);
    // If there would be any schema to be registered, then it can be done after the
    // initialization, but before the first usage of that schema
    await SchemaValidator.registerSchema(id, schema);
};

// The schema parameter could be a schema URI (either HTTP(S) or (ID)) or a schema definition
async function validate(schema, data) {
    await SchemaValidator.validate(schema, data);
};
```

This validator supports the following references within the schema files (see `$ref` in the example).

| Name    | Example                               |
|---------|---------------------------------------|
| HTTP(S) | `https://example.com/schemas/my.json` |
| ID      | `id://my.example.schema`              |

#### Schema example

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

#### Security

Though it does not use eval, there is a special _Content Security Policy_ setting required by high security environments.

If any `script-src` is set in the _Content-Security-Policy_ header, then a `'wasm-unsafe-eval'` entry must be added into the script-src section of this header. (Details on [MDN](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Content-Security-Policy/script-src#unsafe_webassembly_execution)).

```plain
Content-Security-Policy: ...;script-src your entries 'wasm-unsafe-eval'; ...
```

### Node.js

Thanks to WASM support of Node.js, this module can be used in Node.js environment too.

This is an ESM module. Therefore it can be imported only using `import`, `require` is not supported.

It should be initialized by loading the WASM module first, before any validation.

```js
import { readFile } from "node:fs/promises";
import { SchemaValidator } from "@lsolova/json-schema-validator";

async function sample() {
    const wasmBuffer = await readFile(
        "./node_modules/@lsolova/json-schema-validator/dist/assets/schema_validator.wasm"
    );
    await SchemaValidator.init(wasmBuffer);

    const schemaContent = await readFile("./schemas/my.schema.json", { encoding: "utf8" });
    await SchemaValidator.validate(schemaContent, myContentToValidate);
}
```

## How to add WASM file to your deployment?

### Browser: Using esbuild

TBD

### Browser: Using Vite

Please, use the [explicit URL import](https://vite.dev/guide/assets#explicit-url-imports) feature of Vite passing WASM URL to the _SchemaValidator.init_ function. Everything else will be managed by Vite build.

```ts
import { SchemaValidator } from "@lsolova/json-schema-validator";
import wasmURL from "@lsolova/json-schema-validator/dist/assets/schema_validator.wasm?url";

async function initValidation() {
    await SchemaValidator.init(wasmURL);
}
```

### Node.js

The _SchemaValidator.init_ supports passing a module instead of a path. Therefore WASM can be read from file and passed to the init function.

```js
import { readFile } from "node:fs/promises";
import { SchemaValidator } from "@lsolova/json-schema-validator";

async function sample() {
    const wasmBuffer = await fs.readFile(
        "./node_modules/@lsolova/json-schema-validator/dist/assets/schema_validator.wasm"
    );
    await SchemaValidator.init(wasmBuffer);
}
```

## Development

1. Run `npm i`
2. Run `npm run build`

### Local testing

To start the web application on localhost, run `npm run serve`.

To check the Node.js run on localhost, run `npx tsx node-example.ts`.
