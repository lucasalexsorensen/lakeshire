import React from "react";
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import { faRotateLeft, faSave, faArrowsRotate, faTrash } from "@fortawesome/free-solid-svg-icons";
import CanvasDraw from "react-canvas-draw";
import { Store } from "tauri-plugin-store-api";
import { ZoneInfo } from "../types/Zone";


type EditorMapProps = {
    currentZoneInfo: ZoneInfo;
    handleSave: (x: any) => void;
}

type EditorMapState = {
    savedDrawDataString: string
}

const store = new Store(".settings.dat");

export default class EditorMap extends React.Component<EditorMapProps> {
    canvasRef: any;

    handleSave () {
        return this.props.handleSave(this.canvasRef.getSaveData());
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

    async componentDidMount () {
        const { currentZoneInfo } = this.props;
        const savedDrawDataString = await store.get(`draw-data-${currentZoneInfo.name}`);
        if (savedDrawDataString) {
            this.canvasRef.loadSaveData(JSON.stringify(savedDrawDataString), false);
        }
    }


    render () {
        const { currentZoneInfo } = this.props;
        const imagePath = `/zones/${currentZoneInfo.imageName ?? currentZoneInfo.name}.jpg`;

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
