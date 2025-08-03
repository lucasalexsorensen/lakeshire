from abc import ABC, abstractmethod

from lakeshire.Lakeshire_pb2 import GameState as GameStatePb
from lakeshire.models import GameState, PlayerInfo, PosInfo, TargetInfo


class Parser(ABC):
    @abstractmethod
    def parse(self) -> GameState:
        pass


class DefaultParser(Parser):
    def parse(self, bytes: bytes) -> GameState:
        game_state_pb = GameStatePb()
        game_state_pb.ParseFromString(bytes)

        pos_info_pb = game_state_pb.Player.PosInfo

        game_state = GameState(
            player=PlayerInfo(
                pos=PosInfo(
                    map_coords=(
                        pos_info_pb.MapX / 1e14,
                        pos_info_pb.MapY / 1e14,
                    ),
                    world_coords=(
                        pos_info_pb.WorldX / 1e5,
                        pos_info_pb.WorldY / 1e5,
                    ),
                    facing=pos_info_pb.Facing / 1e10,
                    map_id=pos_info_pb.MapId,
                    instance_id=pos_info_pb.InstanceId,
                )
            ),
            target=TargetInfo(),
        )
        return game_state
