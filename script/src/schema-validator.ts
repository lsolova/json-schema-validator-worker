import wasmInit, { WasmSchemaValidator } from "../wasm/schema_validator.js";
import { WasmStatus, WasmStatusSet } from "./types.js";

export class SchemaValidator {
    private _wasmStatus: WasmStatus = WasmStatusSet.UNINITIALIZED;
    private _schemaValidator: WasmSchemaValidator | null = null;

    async init(wasm: string | Uint8Array): Promise<void> {
        if (this._wasmStatus === WasmStatusSet.UNINITIALIZED || this._wasmStatus === WasmStatusSet.FAILED) {
            try {
                await wasmInit({module_or_path: wasm});
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
