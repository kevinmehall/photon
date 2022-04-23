import * as preact from "preact";
import { useUrl } from "./util";
import { HomeView } from "./HomeView";
import { DatasetView } from "./DatasetView";

function App() {
    const [url, setUrl] = useUrl();

    const urlParts = url.split('/').filter((p) => p != '')
    const datasetName = urlParts[0];

    if (datasetName) {
        return (<DatasetView datasetName={datasetName} onChangeDataset={() => setUrl('/')} />);
    } else {
        return (<HomeView selectDataset={(n) => setUrl(`/${n}`)}/>);
    }
}

preact.render(<App />, document.body); 