import { SchemaValidator as SchemaValidatorClass } from "./schema-validator";

const schemaValidator = new SchemaValidatorClass();

export const SchemaValidator = {
    /** Initializing the SchemaValidator by loading the underlying WASM module.
     *
     *  @param wasm The absolute or relative URL of the WASM module or the binary (UInt8Array) content of the WASM file
     */
    init: schemaValidator.init.bind(schemaValidator),
    /** Registering a schema with a custom identifier. This id can be used for the validation afterwards.
     *
     *  @param id Identifier of the schema, prefixed with `id://` protocol.
     *  @param schema Content of the schema. It can be provided as string or an object.
     */
    registerSchema: schemaValidator.registerSchema.bind(schemaValidator),
    /** Validates the data with the provided schema (or the schema related to the provided URI).
     *
     *  @param schema A schema URI (HTTP(S) or ID) or the schema itself.
     *  @param data The data to be validated against the schema.
     *  @returns Promise<boolean> true if the data is valid
     *  @throws An error if the data is invalid. This error contains additional details of the first validation error.
     */
    validate: schemaValidator.validate.bind(schemaValidator),
};
