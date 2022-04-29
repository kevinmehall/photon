import { useEffect, useReducer } from "preact/hooks";
import { useEvent } from "./util";
import { Filter } from "./api";

export type State = {
    fields: Array<string>;
    filter: { [key: string]: Filter };
};

const initialState: State = {
    fields: [],
    filter: {},
};

export type Action =
    | { type: 'replace', state: State }
    | { type: 'addField', field: string }
    | { type: 'removeField', field: string }
    | { type: 'filterKeyword', field: string, value: string, include: boolean }
    | { type: 'filterKeywordSet', field: string, values: string[], include: boolean }
    ;

export type DispatchFn = (_: Action) => void;

function reducer(state: State, action: Action): State {
    switch (action.type) {
        case 'replace':
            return { ...state, ...action.state };
        case 'addField':
            if (state.fields.includes(action.field)) {
                return state;
            }
            return { ...state, fields: [...state.fields, action.field] };
        case 'removeField':
            return { ...state, fields: state.fields.filter(e => e != action.field) };
        case 'filterKeyword':
            return { ...state, filter: { 
                ...state.filter,
                [action.field]: updateKeywordFilter(state.filter[action.field], action.value, action.include)
            }};
        case 'filterKeywordSet':
             return { ...state, filter: { 
                ...state.filter,
                [action.field]: action.include ? { is: action.values } : { not: action.values }
            }};

    }
}

function addRemove<T>(arr: T[], val: T, include: boolean): T[] {
    if (include) {
        return arr.includes(val) ? arr : [...arr, val];
    } else {
        return arr.filter((x) => x != val);
    }
}

function updateKeywordFilter(filter: Filter, value: string, include: boolean): Filter {
    if (filter && "is" in filter) {
        const v = addRemove(filter.is, value, include);
        return v.length ? { is : v } : undefined;
    } else if (include) {
        return { is: [value] };
    } else if (filter && "not" in filter) {
        const v = addRemove(filter.not, value, !include);
        return v.length ? { not: v } : undefined;
    } else {
        return { not: [value] };
    }
}

function hashString(): string {
    return decodeURIComponent(window.location.hash.slice(1))
}

function serialize(state: State): string {
    return JSON.stringify(state)
}

function deserialize(url: string): State {
    return url && JSON.parse(url)
}

export function usePhotonState(): [State, DispatchFn] {
    const [state, dispatch] = useReducer(reducer, null, () => ({...initialState, ...deserialize(hashString())}));

    useEvent(window, 'popstate', () => {
        const v = hashString();
        dispatch({'type': 'replace', 'state': deserialize(v)});
    });

    useEffect(() => {
        window.history.replaceState({}, '', '#' + serialize(state));
    }, [state]);

    return [state, dispatch];
}
