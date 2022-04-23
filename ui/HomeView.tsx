import * as preact from "preact";
import { useReq } from "./req";
import { RootRes } from "./api";

export function HomeView({ selectDataset }: { selectDataset: (n: string) => void; }) {
    const infoRes = useReq<null, RootRes>("get", `/`);

    return (
        <>
            <div id='header'>
                <div id='home-version'>Photon {infoRes.status == 'ok' && (<span>v{infoRes.data.version}</span>)}</div>
            </div>
            <div id='home-dataset-list'>
                {infoRes.status == 'ok' ? Object.entries(infoRes.data.datasets).map(([dsName, ds]) => {
                    return (
                        <button class='home-dataset-list-entry' onClick={() => selectDataset(dsName)}>
                            <h2>{dsName}</h2>
                            {ds.ok ? null : <span class='home-dataset-error'>Configuration error</span>}
                        </button>
                    );
                }) : "Loading..."}
            </div>
        </>
    );
}
