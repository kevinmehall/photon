import * as preact from "preact";
import { State, DispatchFn } from "./state";
import { Field, FieldsRes, Filter } from "./api";
import { Res } from "./req";
import * as Icons from "./icons";
import { classes } from "util";
import { useState } from "preact/hooks";

export type SidebarFieldsProps = {
    fieldsRes: Res<FieldsRes>,
    state: State,
    dispatch: DispatchFn;
}

type FieldProps = {
    fieldName: string,
    field: Field;
    state: State,
    dispatch: DispatchFn,
};

export function SidebarFields({fieldsRes, state, dispatch}: SidebarFieldsProps) {
    let fields: { [key: string]: Field };
    if (fieldsRes.status == 'ok') {
        fields = fieldsRes.data.fields;
    } else {
        return <div>Loading</div>;
    }

    return (
        <div id='fields'>
            {Object.entries(fields).map(([fieldName, field]) => 
                <Field 
                    fieldName={fieldName}
                    field={field}
                    state={state}
                    dispatch={dispatch}
                />
            )}
        </div>
    );
}

function Field({ fieldName, field, state, dispatch }: FieldProps) {
    const inTable = state.fields.includes(fieldName);
    return (
        <details class={classes({ field: true })}>
            <summary>
                <h3>
                    <span class='fieldName'>{fieldName}</span>
                    <button
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
            </summary>
            <input class='filter-add' type='text' onKeyUp={(e) => {
                if (e.code == "Enter" && e.currentTarget.value != '') {
                    dispatch({ type: 'filterKeyword', field: fieldName, value: e.currentTarget.value, include: !e.shiftKey });
                    e.currentTarget.value = '';
                }
             }} />
        </details>
    );
}

type FilterProps = { filter: Filter };

function FilterList({filter}: FilterProps) {
    if (filter && "is" in filter) {
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
