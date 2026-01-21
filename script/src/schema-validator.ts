import wasmInit, { WasmSchemaValidator } from "../wasm/schema_validator.js";
import { WasmStatus, WasmStatusSet } from "./types.js";

export class SchemaValidator {
    private _wasmStatus: WasmStatus = WasmStatusSet.UNINITIALIZED;
    private _schemaValidator: WasmSchemaValidator | null = null;

    /** Initializing the SchemaValidator by loading the underlying WASM module.
     *
     *  @param wasmURL The absolute or relative URL of the WASM module
     */
    async init(wasmURL: string): Promise<void> {
        if (this._wasmStatus === WasmStatusSet.UNINITIALIZED || this._wasmStatus === WasmStatusSet.FAILED) {
            try {
                await wasmInit({module_or_path: wasmURL});
                this._schemaValidator = new WasmSchemaValidator();
                this._wasmStatus = WasmStatusSet.READY;
            } catch (error) {
                this._wasmStatus = WasmStatusSet.FAILED;
                throw new Error(`WASM initialization failed. ${JSON.stringify(error)}`);
            }
        }
    }

    /** Registering a schema with a custom identifier. This id can be used for the validation afterwards.
     *
     *  @param id Identifier of the schema, prefixed with `id://` protocol.
     *  @param schema Content of the schema. It can be provided as string or an object.
     */
    async registerSchema(id: string, schema: string | object): Promise<void> {
        if (this._wasmStatus !== WasmStatusSet.READY || this._schemaValidator === null) {
            throw new Error("WASM is not initialized. Call init() first.");
        }

        try {
            const schemaContent = typeof schema === "string" ? schema : JSON.stringify(schema);
            await this._schemaValidator.add_schema(id, schemaContent);
        } catch (error) {
            throw error instanceof Error ? error.message : new Error(JSON.stringify(error));
        }
    }

    /** Validates the data with the provided schema (or the schema related to the provided URI).
     *
     *  @param schema A schema URI (HTTP(S) or ID) or the schema itself.
     *  @param data The data to be validated against the schema.
     *  @returns Promise<boolean> true if the data is valid
     *  @throws An error if the data is invalid. This error can contain additional details of the validity check result.
     */
    async validate(schema: string | object, data: string | object): Promise<boolean> {
        if (this._wasmStatus !== WasmStatusSet.READY || this._schemaValidator === null) {
            throw new Error("WASM is not initialized. Call init() first.");
        }

        try {
            const schemaOrUri = typeof schema === "string" ? schema : JSON.stringify(schema);
            const dataString = typeof data === "string" ? data : JSON.stringify(data);
            const isValid = await this._schemaValidator.validate(schemaOrUri, dataString);
            return isValid;
        } catch (error) {
            throw error instanceof Error ? error.message : new Error(JSON.stringify(error));
        }
    }
}
