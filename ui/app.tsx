import * as preact from "preact";
import { SidebarFields } from "./Sidebar";
import { Table } from "./Table";
import { useReq } from "./req";
import * as Icons from "./icons";
import { FieldsRes, QueryReq, QueryRes } from "./api";
import { useUrl } from "./util";
import { usePhotonState } from "./state";

function App() {
    const [url, setUrl] = useUrl();

    const urlParts = url.split('/').filter((p) => p != '')
    const datasetName = urlParts[0] || 'default'

    const fields = useReq<null, FieldsRes>("get", `/${datasetName}/_fields`)
    const [state, dispatch] = usePhotonState();

    const data = useReq<QueryReq, QueryRes>('post', `/${datasetName}/_query`, {
        filter: state.filter,
        returning: state.fields,
    });

    return (
        <>
            <div id='header'>
                <button id='source' onClick={() => setUrl('/')}>
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

preact.render(<App />, document.body); 