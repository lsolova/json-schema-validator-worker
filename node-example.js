import { readFile } from "node:fs/promises";
import { SchemaValidator } from "./dist/index.js";

async function validateWithWasm() {
    try {
        const wasmPath = "./dist/assets/schema_validator.wasm";
        const wasmBuffer = await readFile(wasmPath);
        await SchemaValidator.init(wasmBuffer);

        const schema = readFile("./public/test.schema.json");
        const data = {
            name: "John",
            age: 30,
            email: "john@example.com"
        };

        const result = await SchemaValidator.validate(schema, data);
        if (result) {
            console.info("Schema is valid.");
        }
    } catch (error) {
        console.error("Failed", error);
    }
}

validateWithWasm();
