import * as preact from "preact";
import { State, DispatchFn, Action } from "./state";
import { Field, FieldsRes, Filter } from "./api";
import { Res } from "./req";
import * as Icons from "./icons";
import { classes } from "util";
import { useState } from "preact/hooks";

export type SidebarProps = {
    fields: FieldsRes,
    state: State,
    dispatch: DispatchFn;
}

export function Sidebar({fields, state, dispatch}: SidebarProps) {
    const [searchText, setSearch] = useState('');

    const searchParse = searchText.match(/^([a-zA-Z0-9\/._-]*)\s*(?:([!:=@~]+[*]*)\s*(.*))?$/) || [];
    const searchField = searchParse[1] || '';
    const searchOp = searchParse[2];
    const searchArg = searchParse[3];

    const filteredFields = Object.entries(fields.fields)
        .filter(([field, _]) => field.indexOf(searchField) != -1);

    let searchAction: Action;
    if (fields.fields.hasOwnProperty(searchField) && searchOp) {
        if (searchOp == ":" && searchArg) {
            searchAction = { 'type': 'filterKeywordSet', 'field': searchField, 'values': searchArg.split(','), include: true };
        } else if (searchOp == ':*') {
            searchAction = { 'type': 'filterPresent', 'field': searchField, present: true };
        } else if (searchOp == '!' && searchArg) {
            searchAction = { 'type': 'filterKeywordSet', 'field': searchField, 'values': searchArg.split(','), include: false };
        } else if (searchOp == '!*') {
            searchAction = { 'type': 'filterPresent', 'field': searchField, present: false };
        }
    }

    return (<>
        <div class='row'>
            <input
                id='search'
                placeholder='Search fields'
                value={searchText}
                onInput={(e) => setSearch(e.currentTarget.value)}
                onKeyDown={(e) => {
                    console.log(e.code)
                    if (e.code == "Enter" || e.code == "Tab") {
                        if (filteredFields.length == 1 && filteredFields[0] && !searchOp) {
                            setSearch(filteredFields[0][0]);
                            e.preventDefault();
                        } else if (e.code == 'Enter' && searchAction) {
                            dispatch(searchAction);
                            e.preventDefault();
                            e.currentTarget.select();
                        }

                    } else if (e.code == "Escape") {
                        setSearch('');
                    }
                }}
            />
            <button 
                id='search-clear'
                onClick={() => setSearch('')}
                class={classes({ hidden: !searchText })}
                title="Clear search"
            >
                <Icons.CloseOutline />
            </button>
        </div>
        <div id='fields'>
            {filteredFields.map(([fieldName, field]) =>
                <Field
                    key = {fieldName}
                    fieldName={fieldName}
                    field={field}
                    selectField = {() => setSearch(fieldName)}
                    selected={fieldName === searchField}
                    state={state}
                    dispatch={dispatch}
                />
            )}
        </div>
    </>);
}


type FieldProps = {
    fieldName: string,
    field: Field;
    state: State,
    selectField: () => void,
    selected: boolean,
    dispatch: DispatchFn,
};

function Field({ fieldName, field, selectField, selected, state, dispatch }: FieldProps) {
    const inTable = state.fields.includes(fieldName);
    return (
        <div class={classes({ field: true, selected })}>
            <h3>
                <button onClick={selectField} class='fieldName'>{fieldName}</button>
                <button
                    class='icon'
                    title={inTable ? "Remove field from table" : "Show field in table"}
                    onClick={(e) => {
                        dispatch({type: inTable ? 'removeField' : 'addField', field: fieldName});
                        e.preventDefault();
                    }}
                >
                    { inTable ? <Icons.RemoveCircle /> : <Icons.AddCircleOutline /> }
                </button>
            </h3>
            <FilterList filter={state.filter[fieldName]} />
        </div>
    );
}

type FilterProps = { filter: Filter };

function FilterList({filter}: FilterProps) {
    if (filter && "present" in filter) {
        return (
            <ul class='filter filter-present'>
                <li>{filter.present ? "present" : "absent"}</li>
            </ul>
        );
    } else if (filter && "is" in filter) {
        return (
            <ul class='filter filter-keyword-is'>
                {filter.is.map((v) => (<li>{v}</li>))}
            </ul>
        );
    } else if (filter && "not" in filter) {
        return (
            <ul class='filter filter-keyword-is-not'>
                {filter.not.map((v) => (<li>{v}</li>))}
            </ul>
        );
    } else if (filter && "min" in filter) {
        return (
            <ul class='filter filter-range'>
                {filter.min} - {filter.max}
            </ul>
        );
    } else {
        return null;
    }
}
