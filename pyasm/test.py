from termcolor import colored
import subprocess
import argparse
import os

parser = argparse.ArgumentParser(description='rasm test script')
parser.add_argument('-r', '--record', help='Record the tests', action='store_true')
parser.add_argument('-t', '--test', help='Run the tests', action='store_true')
parser.add_argument('-a', '--all', help='Records and tests (warning)', action='store_true')
args = parser.parse_args()
if args.all: args.record, args.test = True, True

print(colored("Building rasm...", "cyan"))
subprocess.run(["cargo", "build", "--release"])
print(colored("Built rasm!\n", "cyan"))

# TODO : ignore difference between \r\n and \n

if args.record:
    print(colored("Recording tests...", "cyan"))
    for dossier in os.listdir("tests"):
        print(colored(f"Recording {dossier}", "white"))
        subprocess.run(["./target/release/rasm", "-f", f"tests/{dossier}/{dossier}.pyasm", "-c"])
        output = subprocess.run(["./output/output"], capture_output=True)
        # si tests/dossier/out n'existe pas, on le crée
        if not os.path.exists(f"tests/{dossier}/out"):
            os.mkdir(f"tests/{dossier}/out")
        open(f"tests/{dossier}/out/{dossier}.stdout", "wb").write(output.stdout)
        open(f"tests/{dossier}/out/{dossier}.stderr", "wb").write(output.stderr)
        open(f"tests/{dossier}/out/{dossier}.return", "w").write(str(output.returncode))
        print(colored(f"Recorded {dossier}", "green"))
    print(colored("Recorded all tests !", "cyan"))
    
# TODO : add simulating to tests
# TODO : handle errors
# TODO : handle infinite loop
if args.test:
    # compile mode
    if args.record:print()
    print(colored("Testing compiled mode...", "cyan"))
    for dossier in os.listdir("tests"):
        print(colored(f"Testing {dossier}", "white"))
        subprocess.run(["./target/release/rasm", "-f", f"tests/{dossier}/{dossier}.pyasm", "-c"])
        output = subprocess.run(["./output/output"], capture_output=True)
        got_error = False
        if output.returncode != int(open(f"tests/{dossier}/out/{dossier}.return", "r").read()):
            print(colored(f"Test {dossier} failed on return code", "red"))
            print("Should be", open(f"tests/{dossier}/out/{dossier}.return", "r").read(), "but is", output.returncode)
            got_error = True
        if output.stdout != open(f"tests/{dossier}/out/{dossier}.stdout", "rb").read():
            print(colored(f"Test {dossier} failed on stdout", "red"))
            print("Should be", open(f"tests/{dossier}/out/{dossier}.stdout", "rb").read(), "but is", output.stdout)
            got_error = True
        if output.stderr != open(f"tests/{dossier}/out/{dossier}.stderr", "rb").read():
            print(colored(f"Test {dossier} failed on stderr", "red"))
            print("Should be", open(f"tests/{dossier}/out/{dossier}.stderr", "rb").read(), "but is", output.stderr)
            got_error = True
        if not got_error:
            print(colored(f"Test {dossier} passed", "green"))
    print(colored("Ran all tests in compile mode !\n", "cyan"))
    # sim mode
    if args.record:print()
    print(colored("Testing simulated mode...", "cyan"))
    for dossier in os.listdir("tests"):
        print(colored(f"Testing {dossier}", "white"))
        output = subprocess.run(["./target/release/rasm", "-f", f"tests/{dossier}/{dossier}.pyasm", "-s"], stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        got_error = False
        # stderr first to check for not yet implemented
        if output.stderr != open(f"tests/{dossier}/out/{dossier}.stderr", "rb").read():
            # TODO : remove this part for not yet implemented
            if b"thread 'main' panicked at 'not yet implemented'" in output.stderr:
                print(colored(f"Test {dossier} failed beacause of a non yet implemented feature", "yellow"))
                continue
            print(colored(f"Test {dossier} failed on stderr", "red"))
            print("Should be", open(f"tests/{dossier}/out/{dossier}.stderr", "rb").read(), "but is", output.stderr)
            got_error = True
        if output.returncode != int(open(f"tests/{dossier}/out/{dossier}.return", "r").read()):
            print(colored(f"Test {dossier} failed on return code", "red"))
            print("Should be", open(f"tests/{dossier}/out/{dossier}.return", "r").read(), "but is", output.returncode)
            got_error = True
        if output.stdout != open(f"tests/{dossier}/out/{dossier}.stdout", "rb").read():
            print(colored(f"Test {dossier} failed on stdout", "red"))
            print("Should be", open(f"tests/{dossier}/out/{dossier}.stdout", "rb").read(), "but is", output.stdout)
            got_error = True
        if not got_error:
            print(colored(f"Test {dossier} passed", "green"))
    print(colored("Ran all tests in sim mode !", "cyan"))
