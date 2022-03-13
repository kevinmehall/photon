export type Field = {};

export type Filter =
    | {}
    | { eq: string | number }
    | { min?: number; max?: number }
    ;

export type FieldsRes = { fields: { [key: string]: Field } };

export type QueryRes = { results: Array<{ [key: string]: string }> };
export type QueryReq = {
    filter: {},
    returning: Array<string>,
};