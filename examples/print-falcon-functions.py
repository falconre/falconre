import falconre
import log
import sys

elf = falconre.falcon.loader.Elf(sys.argv[1])

program = elf.program_recursive()

for function in program.functions:
    print("{:x}: {}".format(function.address, function.name))