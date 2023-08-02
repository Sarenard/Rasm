import logging
LOG_LEVEL = logging.DEBUG
LOGFORMAT = "  %(log_color)s%(levelname)-8s%(reset)s | %(log_color)s%(message)s%(reset)s"
from colorlog import ColoredFormatter
logging.root.setLevel(LOG_LEVEL)
formatter = ColoredFormatter(LOGFORMAT)
stream = logging.StreamHandler()
stream.setLevel(LOG_LEVEL)
stream.setFormatter(formatter)
log = logging.getLogger('pythonConfig')
log.setLevel(LOG_LEVEL)
log.addHandler(stream)
# log.debug("A quirky message only developers care about")
# log.info("Curious users might want to know this")
# log.warning("Something is wrong and any user should be informed")
# log.error("Serious stuff, this is red for a reason")
# log.critical("OH NO everything is on fire")

import sys
file_input = sys.argv[1]
log.debug(f"Input file : {file_input}")

output_asm = open("output/output.asm", "w")

test = """\
section  .text
  global _start       ;must be declared for using gcc
 
_start:                     ;tell linker entry point
  mov  edx, len    ;message length
  mov  ecx, msg    ;message to write
  mov  ebx, 1      ;file descriptor (stdout)
  mov  eax, 4      ;system call number (sys_write)
  int  0x80        ;call kernel
  mov  eax, 1      ;system call number (sys_exit)
  int  0x80        ;call kernel

section  .data

    msg  db  'Hello, world!',0xa  ;our dear string
    len  equ  $ - msg      ;length of our dear string
"""

output_asm.write(test)

# je suis tellement désolé
import pyautogui
import time
command = lambda command : exec(f"pyautogui.write(\"{command}\")\npyautogui.press('enter')", locals(), globals())
pyautogui.press("win")
command("cmd")
time.sleep(0.2)
command("ubuntu2004.exe")
time.sleep(0.2)
command("nasm -f elf64 /mnt/c/users/user/desktop/pyasm/output/output.asm -o /mnt/c/users/user/desktop/pyasm/output/output.o")
time.sleep(0.2)
command("ld /mnt/c/users/user/desktop/pyasm/output/output.o -o /mnt/c/users/user/desktop/pyasm/output/output")
time.sleep(0.2)
command("/mnt/c/users/user/desktop/pyasm/output/output")