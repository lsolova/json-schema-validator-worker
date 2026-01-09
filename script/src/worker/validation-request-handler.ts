import { validate } from "../../wasm/schema_validator.js";
import { type ErrorResponse, MessageCodeSet, MessageTypeSet, type ValidationRequest, type ValidationResponse } from "./types.js";

export async function handleValidationRequest(message: ValidationRequest
): Promise<ValidationResponse | ErrorResponse> {
    const { id, schemaURL, data } = message;

    let schema = JSON.stringify({ "$ref": schemaURL });
    let validationResponse: ValidationResponse | ErrorResponse;

    try {
        const isValid = await validate(schema, JSON.stringify(data));
        validationResponse = { type: MessageTypeSet.VALIDATION_RESULT, id, isValid };
    } catch (error) {
        validationResponse = { type: MessageTypeSet.ERROR, id, code: MessageCodeSet.VALIDATION_FAILED, message: error instanceof Error ? error.message : JSON.stringify(error) };
    }
    return validationResponse;
}
