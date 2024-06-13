export enum ZoneCategory {
    EasternKingdoms = 0,
    Kalimdor = 1,
    Dungeon = 2,
}

export type ZoneInfo = {
    category: ZoneCategory
    id: number,
    name: string,
}
