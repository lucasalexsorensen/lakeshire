import queue
import threading
import time
from dataclasses import dataclass
from datetime import datetime
from pathlib import Path

import click
import matplotlib.animation as animation
import matplotlib.pyplot as plt
import numpy as np
import pygetwindow as pgw
from PIL import Image
from pynput.keyboard import Controller

from lakeshire.exceptions import NoControlPixelFound
from lakeshire.screen import DefaultParser, DefaultScanner, get_grabber

X_RANGE_SECONDS = 10

LEFT_KEY = "a"
RIGHT_KEY = "d"
FORWARD_KEY = "w"

FACING_THRESHOLD = 0.05
DISTANCE_THRESHOLD = 0.0005


@dataclass
class PositionUpdated:
    facing: float
    x: float
    y: float


@dataclass
class TargetUpdated:
    target_facing: float
    target_x: float
    target_y: float


@click.command("plot")
def plot():
    data_queue = queue.Queue[PositionUpdated | TargetUpdated]()

    grabber = get_grabber()

    durotar_img = Image.open(
        Path(__file__).parent.parent.parent.parent / "bot/static/areas/14.png"
    )

    fig, (img_ax, ax) = plt.subplots(
        2, 1, figsize=(12, 10), gridspec_kw={"height_ratios": [3, 1]}
    )
    plt.tight_layout()
    ax.set_ylim(0, 2 * np.pi)
    ax.set_ylabel("Facing (radians)")
    ax.set_xticks([])
    ax.grid(True)

    img_ax.imshow(durotar_img)
    img_ax.axis("off")

    img_height, img_width = durotar_img.size[1], durotar_img.size[0]
    (position_marker,) = img_ax.plot(
        [], [], "ro", markersize=4, label="Player Position"
    )

    (target_marker,) = img_ax.plot([], [], "go", markersize=5, label="Target Position")

    (path_line,) = img_ax.plot(
        [], [], "g--", linewidth=2, alpha=0.7, label="Path to Target"
    )

    def on_image_click(event):
        nonlocal \
            target_position, \
            data_queue, \
            current_player_pos, \
            target_marker, \
            path_line
        if event.inaxes == img_ax:
            img_height, img_width = durotar_img.size[1], durotar_img.size[0]

            if event.xdata is not None and event.ydata is not None:
                norm_x = event.xdata / img_width
                norm_y = event.ydata / img_height

                if current_player_pos is None:
                    return

                target_position = [norm_x, norm_y]

                target_map_x = norm_x * img_width
                target_map_y = norm_y * img_height
                target_marker.set_data([target_map_x], [target_map_y])

                if current_player_pos is not None:
                    current_map_x = current_player_pos[0] * img_width
                    current_map_y = current_player_pos[1] * img_height
                    path_line.set_data(
                        [current_map_x, target_map_x], [current_map_y, target_map_y]
                    )

                data_queue.put(
                    TargetUpdated(
                        target_facing=0,
                        target_x=norm_x,
                        target_y=norm_y,
                    )
                )

    fig.canvas.mpl_connect("button_press_event", on_image_click)

    x_data = np.array([])
    y_data = np.array([])
    target_position = None
    current_player_pos = None

    (line,) = ax.plot(
        x_data,
        y_data,
        "o-",
        label="Facing",
    )

    target_line = ax.axhline(0, color="red", linestyle="--", visible=False)

    keyboard = Controller()
    pressed_key = None

    def agent_loop():
        nonlocal target_position, grabber, keyboard, pressed_key, data_queue
        while True:
            raw_bytes = None
            while raw_bytes is None:
                img = grabber.grab()
                try:
                    raw_bytes = DefaultScanner(img).scan()
                except NoControlPixelFound:
                    time.sleep(1)
                except Exception as e:
                    raise e
            game_state = DefaultParser().parse(raw_bytes)
            player_pos = game_state.player.pos
            data_queue.put(
                PositionUpdated(
                    facing=player_pos.facing,
                    x=player_pos.map_coords[0],
                    y=player_pos.map_coords[1],
                )
            )

            is_wow_active = "world of warcraft" in pgw.getActiveWindow().title().lower()

            if is_wow_active and target_position is not None:
                current_pos = [player_pos.map_coords[0], player_pos.map_coords[1]]
                distance_to_target = np.sqrt(
                    (target_position[0] - current_pos[0]) ** 2
                    + (target_position[1] - current_pos[1]) ** 2
                )

                dx = target_position[0] - current_pos[0]
                dy = current_pos[1] - target_position[1]

                standard_angle = np.arctan2(dy, dx)

                required_facing = (standard_angle + 3 * np.pi / 2) % (2 * np.pi)

                data_queue.put(
                    TargetUpdated(
                        target_facing=required_facing,
                        target_x=target_position[0],
                        target_y=target_position[1],
                    )
                )

                facing_delta = required_facing - player_pos.facing

                if facing_delta > np.pi:
                    facing_delta -= 2 * np.pi
                elif facing_delta < -np.pi:
                    facing_delta += 2 * np.pi

                key_to_press = None
                if abs(facing_delta) >= FACING_THRESHOLD:
                    key_to_press = LEFT_KEY if facing_delta > 0 else RIGHT_KEY
                elif distance_to_target > DISTANCE_THRESHOLD:
                    key_to_press = FORWARD_KEY

                if key_to_press is None:
                    if pressed_key is not None:
                        keyboard.release(pressed_key)
                        pressed_key = None
                else:
                    if pressed_key is None or pressed_key != key_to_press:
                        if pressed_key is not None:
                            keyboard.release(pressed_key)
                        keyboard.press(key_to_press)
                        pressed_key = key_to_press
            else:
                if pressed_key is not None:
                    keyboard.release(pressed_key)
                    pressed_key = None

    def update_loop(_):
        nonlocal \
            y_data, \
            x_data, \
            target_position, \
            data_queue, \
            position_marker, \
            current_player_pos, \
            target_marker, \
            path_line

        try:
            msg = data_queue.get_nowait()
        except queue.Empty:
            return (line, target_line, position_marker, target_marker, path_line)

        if isinstance(msg, PositionUpdated):
            new_facing = msg.facing
            x_data = np.append(x_data, datetime.now().timestamp())[-100:]
            y_data = np.append(y_data, new_facing)[-100:]
            line.set_xdata(x_data)
            line.set_ydata(y_data)
            ax.relim()
            ax.autoscale_view()

            map_x = msg.x * img_width
            map_y = msg.y * img_height
            position_marker.set_data([map_x], [map_y])

            if current_player_pos is None:
                current_player_pos = [msg.x, msg.y]
            else:
                current_player_pos[0] = msg.x
                current_player_pos[1] = msg.y

            if target_position is not None:
                current_map_x = msg.x * img_width
                current_map_y = msg.y * img_height
                target_map_x = target_position[0] * img_width
                target_map_y = target_position[1] * img_height

                path_line.set_data(
                    [current_map_x, target_map_x], [current_map_y, target_map_y]
                )
            else:
                path_line.set_data([], [])

        elif isinstance(msg, TargetUpdated):
            target_position = [msg.target_x, msg.target_y]

            target_line.set_visible(True)
            target_line.set_ydata([msg.target_facing, msg.target_facing])
        return (
            line,
            target_line,
            position_marker,
            target_marker,
            path_line,
        )

    agent_thread = threading.Thread(target=agent_loop, daemon=True)
    agent_thread.start()
    _ = animation.FuncAnimation(fig=fig, func=update_loop, interval=5, blit=True)
    plt.show()
