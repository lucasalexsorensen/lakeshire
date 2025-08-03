import time

import click

from lakeshire.cli.plot import plot


@click.group()
def program():
    pass


program.add_command(plot)
