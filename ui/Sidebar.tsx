import * as preact from "preact";
import { State, DispatchFn } from "./state";
import { Field, FieldsRes } from "./api";
import { Res } from "./req";
import * as Icons from "./icons";

export type SidebarFieldsProps = {
    fieldsRes: Res<FieldsRes>,
    state: State,
    dispatch: DispatchFn;
}

type FieldProps = {
    fieldName: string,
    field: Field;
    state: State,
    dispatch: DispatchFn;
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
            {Object.entries(fields).map(([fieldName, field]) => <Field fieldName={fieldName} field={field} state={state} dispatch={dispatch} />)}
        </div>
    );
}

function Field({ fieldName, field, state, dispatch }: FieldProps) {
    const visible = state.fields.includes(fieldName);
    return (
        <div class='field'>
            <span class='fieldName'>{fieldName}</span>
            <button
                title={visible ? "Remove field from table" : "Show field in table"}
                onClick={() => dispatch({type: visible ? 'removeField' : 'addField', field: fieldName})}
            >
                { visible ? <Icons.RemoveCircle /> : <Icons.AddCircleOutline /> }
            </button>
        </div>
    );
}