import falconre
import log
import sys

def print_calls(function):
    for block in function.blocks():
        for instruction in block.instructions():
            operation = instruction.operation()
            if operation.type() == "call":
                log.magenta("0x{:08x}: {}".format(instruction.address(), operation))

elf = falconre.falcon.loader.Elf(sys.argv[1])

program = elf.program_recursive()
program = falconre.raptor.translate_program_parallel(elf, program)

print("Lifted")

for function in program.functions():
    log.blue("{:X}: {}".format(function.address(), function.name()))
    print_calls(function)