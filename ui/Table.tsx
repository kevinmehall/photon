import * as preact from "preact";
import { Res } from "./req";
import { QueryRes } from "./api";
import { State, DispatchFn } from "./state";
import { useState } from "preact/hooks";
import { classes } from "util";

export type TableProps = {
    state: State,
    dispatch: DispatchFn,
    dataRes: Res<QueryRes>,
};

export function Table({state, dataRes, dispatch}: TableProps) {
    const [dropCol, setDropCol] = useState<undefined | number>(undefined);
    const [resizeCol, setResizeCol] = useState<undefined | number>(undefined);
    const [widths, setWidths] = useState < { [key: string]: number }>(() => JSON.parse(localStorage.photonFieldWidths || '{}'));

    let data;
    if (dataRes.status == 'ok') {
        data = dataRes.data.results;
    } else {
        return <div>Loading</div>;
    }

    const defaultWidth = 200;
    const actionsWidth = 32;
    const totalWidth = state.fields.reduce((p, f) => p + (widths[f] || defaultWidth), actionsWidth);

    return (
        <div class='table-wrap'
            onDragOver={(e) => {
                if (e.dataTransfer?.types.includes('photon-field')) {
                    const cell = (e.target as HTMLElement).closest('td,th') as HTMLTableCellElement;
                    if (cell) {
                        const rect = cell.getBoundingClientRect();
                        const idx = Math.max(0, (e.clientX < rect.left + rect.width / 2) ? cell.cellIndex - 1 : cell.cellIndex);
                        setDropCol(idx);
                    } else {
                        setDropCol(state.fields.length);
                    }
                    e.preventDefault();
                }
            }}
            onDragLeave={(e) => setDropCol(undefined)}
            onDrop={(e) => {
                const field = e.dataTransfer?.getData('photon-field');
                if (field != undefined && dropCol != undefined) {
                    dispatch({ type: 'addField', field, pos: dropCol });
                }
                setDropCol(undefined);
            }}
        >
            { state.fields.length ? (<table style={{width: totalWidth}}>
                <colgroup>
                    <col style = {{width: 32}} />
                    { state.fields.map((field) => 
                        <col style={{ width: widths[field] } ?? defaultWidth } />
                    )}
                </colgroup>
                <thead>
                    <tr>
                        <th class='row-actions'>
                            <span class={classes({ handle: true, selected: dropCol === 0 })} />
                        </th>
                        { state.fields.map((field, i) => 
                            <th 
                                draggable={true}
                                onDragStart = { (e) => e.dataTransfer?.setData('photon-field', field) }
                                title={field}
                            >
                                {field}
                                <ResizeHandle
                                    selected={dropCol === i + 1 || resizeCol === i}
                                    onPress={ () => setResizeCol(i) }
                                    onMove={ (width) => setWidths({ ...widths, [field]: Math.max(width, 24) }) }
                                    onRelease={ () => {
                                        localStorage.photonFieldWidths = JSON.stringify(widths);
                                        setResizeCol(undefined);
                                    }}
                                />
                            </th>) }
                    </tr>
                </thead>
                { data.map(row => (
                    <tr>
                        <td></td>
                        { state.fields.map(field => <td>{row[field]}</td>) }
                    </tr>
                )) }
            </table>
            ) : <div class={classes({'table-message': true, 'highlight': dropCol != undefined})}
                >No fields selected. Drag and drop fields to add.
            </div>}
        </div>
    )
}


type ResizeHandleProps = {
    onPress: () => void,
    onMove: (width: number) => void,
    onRelease: () => void,
    selected: boolean,
};
function ResizeHandle({ onPress, onMove, onRelease, selected }: ResizeHandleProps) {
    return (
        <span class={classes({ handle: true, selected })}
            onMouseDown={(e) => {
                const left = e.currentTarget.parentElement?.getBoundingClientRect().left;
                if (left == null) return;

                const move = (e: MouseEvent) => { onMove(e.clientX - left); };

                document.body.style.cursor = 'col-resize';

                const release = (e: MouseEvent) => {
                    document.body.style.cursor = 'auto';
                    document.removeEventListener('mousemove', move);
                    document.removeEventListener('mouseup', release);
                    onRelease();
                }

                document.addEventListener('mousemove', move);
                document.addEventListener('mouseup', release);
                e.preventDefault();
            }}
        />
    );
}
