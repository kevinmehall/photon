import { Filter } from "./api";

export type State = {
    filter: { [key: string]: Filter };
    fields: Array<string>;
};

export const initialState: State = {
    filter: {},
    fields: [],
};

export type Action =
    | { type: 'addField', field: string }
    | { type: 'removeField', field: string }
    ;

export type DispatchFn = (_: Action) => void;

export function reducer(state: State, action: Action): State {
    switch (action.type) {
        case 'addField':
            if (state.fields.includes(action.field)) {
                return state;
            }
            return { ...state, fields: [...state.fields, action.field] };
        case 'removeField':
            return { ...state, fields: state.fields.filter(e => e != action.field) };
    }
    return state;
}
