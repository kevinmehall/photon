import * as preact from "preact";
import { Res } from "./req";
import { QueryRes } from "./api";
import { State, DispatchFn } from "./state";

export type TableProps = {
    state: State,
    dispatch: DispatchFn,
    dataRes: Res<QueryRes>,
};

export function Table({state, dataRes, dispatch}: TableProps) {
    let data;
    if (dataRes.status == 'ok') {
        data = dataRes.data.results;
    } else {
        return <div>Loading</div>;
    }

    return (
        <table>
            <tr>
                { state.fields.map(field => <th>{field}</th>) }
            </tr>
            { data.map(row => (
                <tr>
                    { state.fields.map(field => <td>{row[field]}</td>) }
                </tr>
            )) }
        </table>
    )
}