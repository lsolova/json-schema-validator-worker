import init from "../../wasm/schema_validator.js";
import { type ErrorResponse, type IncomingMessage, isInitializeRequest, isValidationRequest, MessageCodeSet, MessageTypeSet } from "./types.js";
import { handleValidationRequest } from "./validation-request-handler.js";

let wasmStatus = "initializing";
function checkWasmReady(): boolean {
    if (wasmStatus !== "ready") {
        const errorResponse: ErrorResponse = { type: MessageTypeSet.ERROR, code: MessageCodeSet.VALIDATOR_UNITIALIZED, message: "Validator is not initialized yet. Wait for an initialized or a failed message." };
        self.postMessage(errorResponse);
        return false;
    }
    return true;
}


self.addEventListener("message", async (event: MessageEvent<IncomingMessage>) => {
    if (isInitializeRequest(event.data)) {
        init(event.data.wasmURL).then(() => {
            wasmStatus = "ready";
            self.postMessage({ type: MessageTypeSet.INITIALIZED });
        }).catch((err) => {
            wasmStatus = "failed";
            self.postMessage({ type: MessageTypeSet.ERROR, code: MessageCodeSet.VALIDATOR_INIT_FAILED, message: err.message});
        });
    } else if (isValidationRequest(event.data) && checkWasmReady()) {
        const validationResponse = await handleValidationRequest(event.data);
        self.postMessage(validationResponse);
        return;
    }

    self.postMessage({ type: MessageTypeSet.ERROR, code: MessageCodeSet.INVALID_MESSAGE, message: `Received invalid message format.\n${JSON.stringify(event.data)}` });
});
