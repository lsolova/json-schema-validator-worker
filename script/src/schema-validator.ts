import wasmInit, { WasmSchemaValidator } from "../wasm/schema_validator.js";
import { WasmStatus, WasmStatusSet } from "./types.js";

export class SchemaValidator {
    private _wasmStatus: WasmStatus = WasmStatusSet.UNINITIALIZED;
    private _schemaValidator: WasmSchemaValidator | null = null;

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

    async validate(schemaURL: string, data: object): Promise<boolean> {
        if (this._wasmStatus !== WasmStatusSet.READY || this._schemaValidator === null) {
            throw new Error("WASM is not initialized. Call init() first.");
        }
        if (!schemaURL.startsWith("http://") && !schemaURL.startsWith("https://") && !schemaURL.startsWith("id://")) {
            throw new Error("Value of schemaURL is invalid. It must start with 'http://', 'https://' or 'id://'.");
        }
        try {
            let schema = JSON.stringify({ "$ref": schemaURL });
            let dataString = JSON.stringify(data);
            const isValid = await this._schemaValidator.validate(schema, dataString);
            return isValid;
        } catch (error) {
            throw error instanceof Error ? error.message : new Error(JSON.stringify(error));
        }
    }
}
