import React from "react";
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import { faCheck } from "@fortawesome/free-solid-svg-icons";
import { ZoneInfo } from "../types/zone";
import ZONES from '../assets/zones.json';

function classNames(...classes: any[]) {
    return classes.filter(Boolean).join(' ')
  }

type EditorTreeProps = {
    currentZoneInfo?: ZoneInfo;
    setCurrentZoneInfo: (zoneInfo: ZoneInfo) => void;
}

export default class EditorTree extends React.Component<EditorTreeProps> {
    render () {
        const { currentZoneInfo } = this.props;
        if (!currentZoneInfo) {
            return <div>...</div>;
        }

        return <React.Fragment>
            <div className="flex mb-3 w-full space-x-4 rounded-lg bg-slate-700 justify-around">
                {['Eastern Kingdoms', 'Kalimdor'].map((name, category) => (
                    <button
                        key={category}
                        className={classNames(
                            "rounded-lg py-2 px-3",
                            currentZoneInfo.category == category ? "bg-orange-400" : "hover:bg-white/[0.12]"
                        )}
                        onClick={() => this.props.setCurrentZoneInfo(ZONES.find(x => x.category == category) as ZoneInfo)}
                    >
                        {name}
                    </button>
                ))}
            </div>

            <ul className="overflow-auto w-full divide-y divide-slate-200/5 h-4/5">
                {/* @ts-ignore */}
                {ZONES.filter(x => x.category == currentZoneInfo.category).sort().map(zoneData => (
                    <li
                        key={zoneData.name}
                        onClick={() => this.props.setCurrentZoneInfo(zoneData)}
                        className={classNames(
                            "block py-2 px-3 space-y-2 rounded-lg  hover:cursor-pointer",
                            currentZoneInfo.name === zoneData.name ? "bg-orange-400" : "hover:bg-white/[0.12]"
                        )}
                    >
                        <span>{zoneData.name}</span>
                    </li>
                ))}
            </ul>
        </React.Fragment>
    }
}
