import * as preact from "preact";
import { SidebarFields } from "./Sidebar";
import { Table } from "./Table";
import { useReq } from "./req";
import * as Icons from "./icons";
import { FieldsRes, QueryReq, QueryRes } from "./api";
import { usePhotonState } from "./state";

export type DatasetViewProps = { datasetName: String, onChangeDataset: () => void };

export function DatasetView({ datasetName, onChangeDataset }: DatasetViewProps) {
    const fields = useReq<null, FieldsRes>("get", `/${datasetName}/_fields`);
    const [state, dispatch] = usePhotonState();

    const data = useReq<QueryReq, QueryRes>('post', `/${datasetName}/_query`, {
        filter: state.filter,
        returning: state.fields
    });

    return (
        <>
            <div id='header'>
                <button id='source' onClick={onChangeDataset}>
                    <span id='source-name'>{datasetName}</span>
                    <Icons.ChevronDown />
                </button>
            </div>
            <div id='sidebar'>
                <SidebarFields fieldsRes={fields} state={state} dispatch={dispatch} />
            </div>
            <div id='data'>
                <Table dataRes={data} state={state} dispatch={dispatch} />
            </div>
        </>
    );

}
