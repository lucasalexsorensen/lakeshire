import React from "react";
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import { faRotateLeft, faSave, faArrowsRotate, faTrash } from "@fortawesome/free-solid-svg-icons";
import { ZoneInfo } from "../types/zone";
import CanvasDraw from "react-canvas-draw";
import { BaseDirectory, writeTextFile, readTextFile, createDir } from "@tauri-apps/api/fs";


type EditorMapProps = {
    currentZoneInfo: ZoneInfo;
    // handleSave: (x: any) => void;
}

type EditorMapState = {
    savedDrawDataString: string
}

export default class EditorMap extends React.Component<EditorMapProps> {
    canvasRef: any;

    async handleSave () {
        const saveData = this.canvasRef.getSaveData()
        await writeTextFile(
            `zone-data-${this.props.currentZoneInfo.id}.json`,
            saveData,
            { dir: BaseDirectory.AppData }
        );
    }

    handleClear ()  {
        this.canvasRef.clear();
    }

    handleResetZoom () {
        this.canvasRef.resetView();
    }

    handleUndo () {
        this.canvasRef.undo();
    }

    async componentDidUpdate(prevProps: EditorMapProps) {
        const { currentZoneInfo } = this.props;
        if (prevProps.currentZoneInfo.name !== currentZoneInfo.name) {
            await this.updateCanvasFromStore(currentZoneInfo);
        }
    }

    async componentDidMount () {
        const { currentZoneInfo } = this.props;
        await this.updateCanvasFromStore(currentZoneInfo);
    }

    async updateCanvasFromStore (zoneInfo: ZoneInfo) {
        try {
            const savedDrawDataString = await readTextFile(
                `zone-data-${zoneInfo.id}.json`,
                { dir: BaseDirectory.AppData }
            );

            this.canvasRef.loadSaveData(savedDrawDataString, false);
        } catch (e) {
            console.error(e)
            this.canvasRef.clear()
        }
    }


    render () {
        const { currentZoneInfo } = this.props;
        const imagePath = `/zone_images/${currentZoneInfo.id}.png`;

        return <React.Fragment>
            <div className="flex py-2 mb-3 px-3 w-1/2 bg-slate-700 space-x-4 rounded-lg justify-around">
                <button onClick={this.handleSave.bind(this)}><FontAwesomeIcon icon={faSave} size="lg" /> Save</button>
                <button onClick={this.handleResetZoom.bind(this)}><FontAwesomeIcon icon={faArrowsRotate} size="lg" /> Reset</button>
                <button onClick={this.handleClear.bind(this)}><FontAwesomeIcon icon={faTrash} size="lg" /> Clear</button>
                <button onClick={this.handleUndo.bind(this)}><FontAwesomeIcon icon={faRotateLeft} size="lg" /> Undo</button>
            </div>
            <CanvasDraw
                ref={(canvasDraw: any) => (this.canvasRef = canvasDraw)}
                // onChange={console.log.bind(this)}
                brushRadius={2}
                lazyRadius={3}
                brushColor={"#f00"}
                catenaryColor={"#faa"}
                imgSrc={imagePath}
                canvasWidth={1200}
                canvasHeight={750}
                enablePanAndZoom={true}
                loadTimeOffset={2}
            />
        </React.Fragment>
    }
}
