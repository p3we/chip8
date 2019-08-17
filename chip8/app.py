import pygame
import time
from pygame.constants import (
    K_0, K_1, K_2, K_3, K_4, K_5, K_6, K_7, K_8, K_9, K_a, K_b, K_c, K_d, K_e, K_f,
    K_SPACE, K_LEFT, K_RIGHT, K_UP, K_DOWN, QUIT, KEYDOWN, KEYUP
)

from array import array
from pygame.mixer import get_init
from argparse import ArgumentParser
from PIL import Image, ImageOps
from chip8.chip8 import Machine


CPU_STEP_PERIOD = (1 / 60) * 1000000.0
TIMER_TICK_PERDIO = (1 / 60) * 1000000000.0
RESOLUTION = (480, 320)

KEYCODES = {
    K_0: 0x0, K_1: 0x1, K_2: 0x2, K_3: 0x3, K_4: 0x4, 
    K_5: 0x5, K_6: 0x6, K_7: 0x7, K_8: 0x8, K_9: 0x9,
    K_a: 0xa, K_b: 0xb, K_c: 0xc, K_d: 0xd, K_e: 0xe, K_f: 0xf,
    K_SPACE: 0x5, K_LEFT: 0x4, K_RIGHT: 0x6, K_DOWN: 0x2, K_UP: 0x8
}

def crate_tone(frequency):
    """Create a tone with step function."""
    sample_rate, size, _channels = get_init()
    sample_num = round(sample_rate / frequency)
    amplitude = 2 ** (abs(size) - 1) - 1
    sampler = ((amplitude if i < sample_num // 2 else -amplitude) for i in range(sample_num))
    samples = array('h', sampler)
    return pygame.mixer.Sound(samples)


def step(machine, screen):
    """Move forward a program execution."""
    op = machine.step()
    if op[1].startswith('DRAW') or op[1].startswith('CLS'):
        size = screen.get_size()
        image = Image.frombuffer('L', (64, 32), machine.framebuffer)
        image = ImageOps.colorize(image, '#111', '#0a0')
        image = image.resize(size, resample=Image.BOX)
        frame = pygame.image.frombuffer(image.tobytes(), size, 'RGB')
        screen.blit(frame, (0, 0))


def tick(machine):
    """Increase timer counters."""
    machine.tick()


def main():
    parser = ArgumentParser('Chip8 emulator')
    parser.add_argument('filename')
    args = parser.parse_args()

    pygame.mixer.pre_init(44100, -16, 1, 1024)
    pygame.init()

    screen = pygame.display.set_mode(RESOLUTION)

    note = crate_tone(440)
    note.set_volume(0.5)

    machine = Machine()
    machine.load(args.filename)
 
    is_running = True
    is_noisy = False
    last_step, last_tick = 0, 0

    while is_running:
        current = time.time_ns()

        for event in pygame.event.get():
            if event.type == QUIT:
                is_running = False
            if event.type == KEYDOWN and event.key in KEYCODES:
                machine.keyevent(KEYCODES[event.key], True)
            if event.type == KEYUP and event.key in KEYCODES:
                machine.keyevent(KEYCODES[event.key], False)
        
        if current - last_step > CPU_STEP_PERIOD:
            step(machine, screen)
            last_step = current

        if current - last_tick >= TIMER_TICK_PERDIO:
            tick(machine)
            last_tick = current

        if machine.st and not is_noisy:
            note.play(-1)
            is_noisy = True
        if not machine.st and is_noisy:
            note.stop()
            is_noisy = False

        pygame.display.flip()
