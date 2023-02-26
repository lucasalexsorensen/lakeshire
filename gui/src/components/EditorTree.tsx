import React from "react";
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import { faCheck } from "@fortawesome/free-solid-svg-icons";

import ZoneData from '../consts/ZoneData'
import { ZoneInfo } from "../types/Zone";

function classNames(...classes: any[]) {
    return classes.filter(Boolean).join(' ')
  }

type EditorTreeProps = {
    currentZoneInfo: ZoneInfo;
    setCurrentZoneInfo: (zoneInfo: ZoneInfo) => void;
}

export default class EditorTree extends React.Component<EditorTreeProps> {
    render () {
        const { currentZoneInfo } = this.props;
        if (!currentZoneInfo.continent || !currentZoneInfo.name) {
            return <div>...</div>;
        }

        return <React.Fragment>
            <div className="flex mb-3 w-full space-x-4 rounded-lg bg-slate-700 justify-around">
                {Object.keys(ZoneData).map((continent) => (
                    <button
                        key={continent}
                        className={classNames(
                            "rounded-lg py-2 px-3",
                            currentZoneInfo.continent == continent ? "bg-orange-400" : "hover:bg-white/[0.12]"
                        )}
                        onClick={() => this.props.setCurrentZoneInfo({
                            continent,
                            // @ts-ignore
                            name: ZoneData[continent][0].name,
                            // @ts-ignore
                            imageName: ZoneData[continent][0].imageName
                        })}
                    >
                        {continent}
                    </button>
                ))}
            </div>

            <ul className="overflow-auto w-full divide-y divide-slate-200/5 h-4/5">
                {/* @ts-ignore */}
                {ZoneData[currentZoneInfo.continent].map(rawZoneData => (
                    <li
                        key={rawZoneData.name}
                        onClick={() => this.props.setCurrentZoneInfo({
                            continent: currentZoneInfo.continent,
                            name: rawZoneData.name,
                            imageName: rawZoneData.imageName
                        })}
                        className={classNames(
                            "block py-2 px-3 space-y-2 rounded-lg  hover:cursor-pointer",
                            currentZoneInfo.name === rawZoneData.name ? "bg-orange-400" : "hover:bg-white/[0.12]"
                        )}
                    >
                        <span>{rawZoneData.name}</span>
                    </li>
                ))}
            </ul>
        </React.Fragment>
    }
}
