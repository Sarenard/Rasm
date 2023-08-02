import subprocess
import argparse
import os

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

parser = argparse.ArgumentParser(description='Lance le convertisseur')
parser.add_argument('-f', dest='file', type=str, help='The input file', required=True)

args = parser.parse_args()

mypy_command = "mypy main.py"
python_command = f"python main.py {args.file}"

# run mypy with subprocess
p = subprocess.Popen(mypy_command, shell=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
if p.stdout is None:
    log.error("Couldnt open a subprocess")
    exit(1)
if p.stdout.read().startswith(b'Success: no issues found in 1 source file'):
    log.info("Mypy passed for the whole project !")
    os.system(python_command)
    log.info("Python passed for the whole project !")
else:
    log.error("Mypy failed for the whole project !")
    os.system(mypy_command)