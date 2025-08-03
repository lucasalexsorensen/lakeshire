from abc import ABC, abstractmethod

import numpy as np

from lakeshire.exceptions import NoControlPixelFound
from lakeshire.models import Direction, Position

CONTROL_PIXEL = {
    "red": 199,
    "green": 99,
}


class Scanner(ABC):
    @staticmethod
    def scan(img: np.ndarray) -> bytes: ...


class DefaultScanner(Scanner):
    def __init__(self, img: np.ndarray):
        self.img = img

    def scan(self) -> bytes:
        assert self.img.shape[2] == 3  # assert RGB (and not RGBA)
        control_pixel_pos = self.find_control_pixel()
        cell_size = 2 * self.measure_walk(control_pixel_pos, Direction.RIGHT) - 1
        return self.read_bytes(control_pixel_pos, cell_size)

    def find_control_pixel(self) -> Position:
        all_control_pixels = np.argwhere(
            (self.img[:, :, 0] == 199) & (self.img[:, :, 1] == 99)
        )
        if len(all_control_pixels) == 0:
            raise NoControlPixelFound()
        selected_pixel = np.median(all_control_pixels, axis=0).astype(int)
        return Position(selected_pixel.tolist())

    def measure_walk(self, pos: Position, direction: Direction) -> int:
        first_observed_color = self.img[pos]
        color = first_observed_color
        steps = 0
        while np.all(color == first_observed_color):
            pos = pos + direction.value
            steps += 1
            color = self.img[pos]
        return steps

    def read_bytes(self, reader_pos: Position, cell_size: int):
        reader_pos += Direction.RIGHT.value * cell_size
        remainder, size_a, size_b = self.img[reader_pos]
        grid_size = int.from_bytes([size_a, size_b], "little")
        reader_pos += Direction.RIGHT.value * cell_size
        grid_dim = int(np.ceil(np.sqrt(grid_size)))

        read_values = []
        i = 3
        while i <= grid_size:
            if i == grid_size:
                if remainder == 0:
                    end_idx = 3
                else:
                    end_idx = remainder
                read_values += self.img[reader_pos][:end_idx].tolist()
            else:
                read_values += self.img[reader_pos].tolist()
            if i % grid_dim == 0:
                # move up & reset to left
                reader_pos += Direction.UP.value * cell_size
                reader_pos -= Direction.RIGHT.value * cell_size * (grid_dim - 1)
            else:
                # move right
                reader_pos += Direction.RIGHT.value * cell_size
            i += 1

        return bytes(read_values)
