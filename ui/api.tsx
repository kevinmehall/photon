export type Field = {};

export type FilterKeywordIs = { is: string[] };
export type FilterKeywordNot = { not: string[] };
export type FilterNumberRange = { min?: number; max?: number };
export type FilterTimeRange = { after?: string, before?: string };

export type Filter =
    | undefined
    | FilterKeywordIs
    | FilterKeywordNot
    | FilterNumberRange
    | FilterTimeRange
    ;

export type FieldsRes = { fields: { [key: string]: Field } };

export type QueryRes = { results: Array<{ [key: string]: string }> };
export type QueryReq = {
    filter: {},
    returning: Array<string>,
};