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
