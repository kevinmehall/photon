export type RootDataset = { ok: boolean };
export type RootRes = {
    version: string,
    datasets: { [key: string]: RootDataset };
}

export type FieldType =
    | 'keyword'
    | 'phrase'
    | 'number'
    | 'timestamp'
    ;

export type Field = {
    type: FieldType,
    values?: [string],
};

export type FilterPresent = { present: boolean };
export type FilterKeywordIs = { is: string[] };
export type FilterKeywordNot = { not: string[] };
export type FilterNumberRange = { min?: number; max?: number };
export type FilterTimeRange = { after: string, before: string };
export type FilterTimeSince = { since: number };

export type Filter =
    | undefined
    | FilterPresent
    | FilterKeywordIs
    | FilterKeywordNot
    | FilterNumberRange
    | FilterTimeRange
    | FilterTimeSince
    ;

export type FieldsRes = { fields: { [key: string]: Field } };

export type QueryRes = { results: Array<{ [key: string]: string }> };
export type QueryReq = {
    filter: {},
    returning: Array<string>,
};