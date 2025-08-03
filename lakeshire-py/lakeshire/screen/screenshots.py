import functools
from abc import ABC, abstractmethod

import numpy as np
from mss import mss


class ScreenshotGrabber(ABC):
    @abstractmethod
    def __init__(self, region: tuple[int, int, int, int]):
        pass

    @abstractmethod
    def grab(self) -> np.ndarray:
        pass


class MSSScreenshotGrabber(ScreenshotGrabber):
    def __init__(self, region: tuple[int, int, int, int]):
        self.mss = mss()

        S = 200
        monitor = self.mss.monitors[0]
        self.region = {
            "left": 0,
            "top": monitor["height"] - S,
            "width": S,
            "height": S,
        }

    def grab(self) -> np.ndarray:
        # convert from BGRA to RGB
        frame = np.array(self.mss.grab(self.region))
        return np.flip(frame[:, :, :3], 2)


@functools.lru_cache(maxsize=1)
def get_grabber() -> ScreenshotGrabber:
    region = (0, 0, 300, 400)
    return MSSScreenshotGrabber(region=region)
