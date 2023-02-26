import React from "react";
import EditorMap from "../components/EditorMap";
import EditorTree from "../components/EditorTree";
import ZoneData from '../consts/ZoneData'
import { ZoneInfo } from "../types/Zone";


type EditorState = {
    currentZoneInfo?: ZoneInfo;
}

class Editor extends React.Component<{}, EditorState> {
    constructor (props: any) {
        super(props);


        const rawZoneData = ZoneData['Kalimdor'].find(x => x.name == 'Durotar')
        if (!rawZoneData) throw new Error('Default zone not found');

        this.state = {
            currentZoneInfo: {
                continent: 'Kalimdor',
                name: rawZoneData.name,
                imageName: rawZoneData.imageName,
            }
        }
    }

    render () {
        const { currentZoneInfo } = this.state;
        const imagePath = `/zones/${currentZoneInfo?.imageName ?? currentZoneInfo?.name}.jpg`;
        if (!currentZoneInfo) return (<div>...</div>);
        return (
            <div className="grid grid-cols-6">
                <div className="col-span-1 flex flex-col items-center p-2">
                    <EditorTree
                        currentZoneInfo={currentZoneInfo}
                        setCurrentZoneInfo={(zoneInfo: ZoneInfo) => {
                            this.setState({ currentZoneInfo: zoneInfo })
                        }}
                    />
                </div>

                <main className="col-span-5 flex flex-col items-center">
                    <EditorMap backgroundImage={imagePath} />
                </main>
            </ div>
        );
    }
}

export default Editor;
