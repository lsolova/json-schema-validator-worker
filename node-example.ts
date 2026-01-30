import { readFile } from "node:fs/promises";
import { SchemaValidator } from "./dist/index.js";

async function validateWithWasm() {
    try {
        const wasmBuffer = await readFile("./dist/assets/schema_validator.wasm");
        await SchemaValidator.init(wasmBuffer);

        const schema = await readFile("./public/test-node.schema.json", { encoding: "utf8" });
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
