from dataclasses import dataclass
from enum import Enum


class Position(tuple):
    def __add__(self, other: "Position") -> "Position":
        return Position((self[0] + other[0], self[1] + other[1]))

    def __sub__(self, other: "Position") -> "Position":
        return Position((self[0] - other[0], self[1] - other[1]))

    def __mul__(self, other: int) -> "Position":
        return Position((self[0] * other, self[1] * other))


class Direction(Enum):
    LEFT = Position((0, -1))
    RIGHT = Position((0, 1))
    UP = Position((-1, -0))
    DOWN = Position((1, 0))


@dataclass
class UnitInfo:
    pass


@dataclass
class PosInfo:
    map_id: int
    instance_id: int
    map_coords: tuple[float, float]
    world_coords: tuple[float, float]
    facing: float


@dataclass
class PlayerInfo:
    # unit: UnitInfo
    pos: PosInfo


@dataclass
class TargetInfo:
    # unit: UnitInfo
    pass


@dataclass
class GameState:
    player: PlayerInfo
    target: TargetInfo | None
