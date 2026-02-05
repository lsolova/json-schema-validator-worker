import { SchemaValidator as SchemaValidatorClass } from "./schema-validator";

const schemaValidator = new SchemaValidatorClass();

export const SchemaValidator = {
    /** Initializing the SchemaValidator by loading the underlying WASM module.
     *
     *  @param wasm string | UInt8Array The absolute or relative URL of the WASM module or the binary content of the WASM file
     */
    init: schemaValidator.init.bind(schemaValidator),
    /** Registering a schema with a custom identifier. This id can be used for the validation afterwards.
     *
     *  @param id string Identifier of the schema, prefixed with `id://` protocol.
     *  @param schema string Content of the schema. It can be provided as string or an object.
     */
    registerSchema: schemaValidator.registerSchema.bind(schemaValidator),
    /** Validates the data with the provided schema (or the schema related to the provided URI).
     *
     *  @param schema string A schema URI (HTTP(S) or ID) or the schema content itself.
     *  @param data string The data to be validated against the schema.
     *  @returns Promise<void> if the data is valid
     *  @throws An error object if the data is invalid. This error contains additional details of the validation in a
     *          key-value pair where key is the field what that error is related to.
     */
    validate: schemaValidator.validate.bind(schemaValidator),
};
