import * as preact from "preact";
import { State, DispatchFn, Action } from "./state";
import { Field, FieldsRes, FieldType, Filter } from "./api";
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

    const searchParse = searchText.match(/^([a-zA-Z0-9\/._-]*)\s*(?:([!:=@~#]+[*]*)\s*(.*))?$/) || [];
    const searchField = searchParse[1] || '';
    const searchOp = searchParse[2];
    const searchArg = searchParse[3];

    const filteredFields = Object.entries(fields.fields)
        .filter(([field, _]) => field.indexOf(searchField) != -1);

    let searchAction: Action | undefined = undefined;
    if (fields.fields.hasOwnProperty(searchField) && searchOp) {
        if (searchOp == ":" && searchArg) {
            searchAction = { 'type': 'filterKeywordSet', 'field': searchField, 'values': searchArg.split(','), include: true };
        } else if (searchOp == ':*') {
            searchAction = { 'type': 'filterPresent', 'field': searchField, present: true };
        } else if (searchOp == '!' && searchArg) {
            searchAction = { 'type': 'filterKeywordSet', 'field': searchField, 'values': searchArg.split(','), include: false };
        } else if (searchOp == '!*') {
            searchAction = { 'type': 'filterPresent', 'field': searchField, present: false };
        } else if (searchOp == "#") {
            searchAction = { 'type': 'filterClear', field: searchField }
        }
    }

    return (<>
        <div class='row'>
            <input
                id='search'
                className={classes({"search-valid": searchAction !== undefined})}
                placeholder='Search fields'
                value={searchText}
                onInput={(e) => setSearch(e.currentTarget.value)}
                onKeyDown={(e) => {
                    console.log(e.code)
                    if (e.code == "Tab") {
                        e.preventDefault();
                        if (filteredFields.length == 1 && filteredFields[0] && !searchOp) {
                            setSearch(filteredFields[0][0]);
                        }
                    } else if (e.code == 'Enter' && searchAction) {
                        dispatch(searchAction);
                        e.currentTarget.select();
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
            <h3
                draggable={true}
                onDragStart={(e) => e.dataTransfer?.setData('photon-field', fieldName) }
            >
                <button onClick={selectField} class='fieldName'><FieldIcon type={field.type} /> {fieldName}</button>
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

            {selected && field.type == 'timestamp' &&
                <FilterEditTime fieldName={fieldName} field={field} filter={state.filter[fieldName]} dispatch={dispatch} /> }
            {selected && field.type == 'keyword' &&
                <FilterEditKeyword fieldName={fieldName} field={field} filter={state.filter[fieldName]} dispatch={dispatch} />}
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
                is: {filter.is.map((v) => (<li>{v}</li>))}
            </ul>
        );
    } else if (filter && "not" in filter) {
        return (
            <ul class='filter filter-keyword-is-not'>
                not: {filter.not.map((v) => (<li>{v}</li>))}
            </ul>
        );
    } else if (filter && "min" in filter) {
        return (
            <ul class='filter filter-range'>
                {filter.min} - {filter.max}
            </ul>
        );
    } else if (filter && "before" in filter) {
        return (<>
            <ul class='filter filter-time'>
                &rarr; <li>{filter.after}</li>
            </ul>
            <ul class='filter filter-time'>
                &larr; <li>{filter.before}</li>
            </ul>
            </>
        );
    } else if (filter && "since" in filter) {
        return (
            <ul class='filter filter-time'>
                last: <li>{formatDuration(filter.since)}</li>
            </ul>
        );
    } else {
        return null;
    }
}

function FieldIcon({ type }: { type: FieldType }) {
    let icon;
    switch (type) {
        case 'keyword':
            icon = <Icons.CodeOutline />;
            break;
        case 'number':
            icon = '#';
            break;
        case 'phrase':
            icon = <Icons.CubeOutline />;
            break;
        case 'timestamp':
            icon = <Icons.TimeOutline />;
            break;
    }

    return (
        <div class='field-icon' data-type={type}>{icon}</div>
    );
}

type FilterEditProps = {
    fieldName: string
    field: Field;
    filter: Filter,
    dispatch: DispatchFn,
};

function FilterEditTime({ fieldName, field, filter, dispatch }: FilterEditProps) {
    const setFilter = (f: Filter) => dispatch({ type: 'filter', field: fieldName, filter: f });

    const [mode, setMode] = useState<"abs" | "rel">((filter && 'after' in filter ? 'abs' : 'rel'));

    let existingAfter, existingBefore;
    if (filter && 'after' in filter && 'before' in filter) {
        existingAfter = new Date(filter.after);
        existingBefore = new Date(filter.before);
    } else if (filter && 'since' in filter) {
        existingBefore = new Date();
        existingAfter = new Date(+existingBefore - filter.since * 1000);
    }

    const [after, setAfter] = useState<Date | undefined>(existingAfter);
    const [before, setBefore] = useState<Date | undefined>(existingBefore);

    let modeToggle = (
        <div class='filter-mode toggle-options'>
            <button onClick={() => setMode('rel')} selected={mode === 'rel'}>Relative</button>
            <button onClick={() => setMode('abs')} selected={mode === 'abs'}>Absolute</button>
        </div>
    );

    let panel;
    if (mode === 'abs') {
        panel = <form onSubmit={(e) => { e.preventDefault(); if (before && after) { setFilter({ before: before.toISOString(), after: after.toISOString() })}}}>
            <DateTimeInput value={after} max={before} setValue={setAfter} zone='utc' />
            <DateTimeInput value={before} min={after} setValue={setBefore} zone='utc' />
            <input type='submit' disabled={!(before && after)} value="Save" />
        </form>
    } else if (mode === 'rel') {
        panel = <div class='filter-time-rel-options options-list'> {
            [
                15 * 60,
                60 * 60,
                6 * 60 * 60,
                24 * 60 * 60,
                7 * 24 * 60 * 60,
                30 * 24 * 60 * 60,
                90 * 24 * 60 * 60,
                365 * 24 * 60 * 60,
            ].map((seconds) =>
                <button onClick={() => setFilter({ since: seconds })} selected={filter && 'since' in filter && filter.since === seconds}>{formatDuration(seconds)}</button>
            )
        } </div>;
    }

    return (
        <div class='filter-edit'>
            {modeToggle}
            {panel}
        </div>
    );
}

function DateTimeInput({value, min, max, setValue, zone}: {value: Date | undefined, min?: Date, max?: Date, setValue: (d: Date | undefined) => void, zone: 'utc'}) {
    function dateInputValue(d: Date | undefined): string | undefined {
        return d && d.toISOString().slice(0, -1)
    }

    function parseDateInputValue(d: string): Date | undefined {
        return d ? new Date(d + 'Z') : undefined;
    }

    return (
        <input type="datetime-local" step='0.001' required
            value={dateInputValue(value)}
            min={dateInputValue(min)}
            max={dateInputValue(max)}
            onChange={(e) => setValue(parseDateInputValue(e.currentTarget.value))}
        />
    );
}

function formatDuration(seconds: number) {
    const qty = (v:number, u:string, us: string) => `${v} ${(v === 1 ? u : us)}`;

    if (seconds % (60 * 60 * 24) == 0) {
        return qty(seconds / 60 / 60 / 24, 'day', 'days')
    } else if (seconds % (60 * 60) == 0) {
        return qty(seconds / 60 / 60, 'hour', 'hours');
    } else if (seconds % 60 == 0) {
        return qty(seconds / 60, 'minute', 'minutes');
    } else {
        return qty(seconds, 'second', 'seconds');
    }
}

function FilterEditKeyword({ fieldName, field, filter, dispatch }: FilterEditProps) {
    const setFilter = (value: string, include: boolean) => dispatch({ type: 'filterKeyword', field: fieldName, value, include });
    const values: string[] = (field.values ?? []).slice();

    const filterIs = filter && 'is' in filter && filter.is;
    const filterNot = filter && 'not' in filter && filter.not;

    (filterIs || filterNot || []).forEach((v) => values.includes(v) || values.push(v));

    return (
        <div class='filter-edit filter-keyword-list options-list'>
            {values.map((v) => (
                <div>
                    {!!(!(filterIs || filterNot) || filterNot && filterNot.includes(v) || filterIs && !filterIs.includes(v)) &&
                        <button title="Include" onClick={() => setFilter(v, true)}>
                            { (filterNot && filterNot.includes(v)) ? <Icons.AddCircle /> : <Icons.AddCircleOutline /> }
                        </button>
                    }
                    {!!(!(filterIs || filterNot) || filterNot && !filterNot.includes(v) || filterIs && filterIs.includes(v)) &&
                        <button title="Exclude" onClick={() => setFilter(v, false)}>
                            { filterIs && filterIs.includes(v) ? <Icons.RemoveCircle /> : <Icons.RemoveCircleOutline /> }
                        </button>
                    }
                    {v}
                </div>
            ))}
        </div>
    );
}