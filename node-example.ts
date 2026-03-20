import { readFile } from "node:fs/promises";
import { SchemaValidator } from "./dist/index.js";

async function validateWithWasm() {
  try {
    const wasmBuffer = await readFile("./dist/assets/schema_validator.wasm");
    await SchemaValidator.init(wasmBuffer);

    const schema1 = await readFile("./public/test-node.schema.json", {
      encoding: "utf8",
    });
    const schema2 = await readFile("./public/email-metadata.schema.json", {
      encoding: "utf8"
    });

    await SchemaValidator.registerSchema("https://example.com/schemas/email-metadata.schema.json", schema2);

    const data1 = {
      name: "John",
      age: 30,
      email: "john@example.com",
    };

    const data2 = {
      from: "john@example.com",
      to: "jane@example.com",
      subject: "Test email"
    }

    await SchemaValidator.validate(schema1, data1);
    console.info("Data 1 is valid.");

    await SchemaValidator.validate("https://example.com/schemas/email-metadata.schema.json", data2);
    console.info("Data 2 is valid.");

    await SchemaValidator.validate("https://json-schema.org/draft/2020-12/schema", schema2);
    console.info("Schema 2 is valid.");
  } catch (error) {
    console.error("Failed", error);
  }
}

validateWithWasm();
