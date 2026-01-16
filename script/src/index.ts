import { SchemaValidator as SchemaValidatorClass } from "./schema-validator";

const schemaValidator = new SchemaValidatorClass();

export const SchemaValidator = {
    init: schemaValidator.init.bind(schemaValidator),
    registerSchema: schemaValidator.registerSchema.bind(schemaValidator),
    validate: schemaValidator.validate.bind(schemaValidator),
};
