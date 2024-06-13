import React from "react";
import EditorMap from "../components/EditorMap";
import EditorTree from "../components/EditorTree";
import { ZoneInfo } from "../types/zone";
import ZONES from '../assets/zones.json';


type EditorState = {
    currentZoneInfo?: ZoneInfo;
}

type DrawData = {
    lines: {
        points: { x: number, y: number }[];
        brushColor: string;
        brushRadius: number;
    }[];
    height: number;
    width: number;
}


class Editor extends React.Component<{}, EditorState> {
    constructor (props: any) {
        super(props);

        // const rawZoneData = ZoneData['Kalimdor'].find(x => x.name == 'Durotar')
        // if (!rawZoneData) throw new Error('Default zone not found');

        this.state = {
            currentZoneInfo: ZONES.find(x => x.name == 'Durotar')
        }
    }

    // async handleSave (data: any) {
    //     const parsedData = JSON.parse(data) as DrawData;
    //     // await store.set(`draw-data-${this.state.currentZoneInfo?.name}`, parsedData);
    // }

    render () {
        const { currentZoneInfo } = this.state;
        if (!currentZoneInfo) return (<div>...</div>);
        return (
            <div className="grid grid-cols-6 p-3">
                <div className="col-span-1 flex flex-col items-center p-2">
                    <EditorTree
                        currentZoneInfo={currentZoneInfo}
                        setCurrentZoneInfo={(zoneInfo: ZoneInfo) => {
                            this.setState({ currentZoneInfo: zoneInfo })
                        }}
                    />
                </div>

                <main className="col-span-5 flex flex-col items-center">
                    <EditorMap
                        currentZoneInfo={currentZoneInfo}
                        // handleSave={this.handleSave.bind(this)}
                    />
                </main>
            </ div>
        );
    }
}

export default Editor;
