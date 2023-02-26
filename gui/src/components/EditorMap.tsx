import React from "react";
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import { faRotateLeft, faSave, faArrowsRotate, faTrash } from "@fortawesome/free-solid-svg-icons";
import CanvasDraw from "react-canvas-draw";


type EditorMapProps = {
    backgroundImage: string | null;
}

export default class EditorMap extends React.Component<EditorMapProps> {
    canvasRef: any;

    handleSave () {
        console.log(JSON.parse(this.canvasRef.getSaveData()));
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


    render () {
        if (!this.props.backgroundImage) {
            return <div>
                Pick a zone to start editing
            </div>;
        }

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
                imgSrc={this.props.backgroundImage}
                canvasWidth={1200}
                canvasHeight={750}
                enablePanAndZoom={true}
            />
        </React.Fragment>
    }
}
