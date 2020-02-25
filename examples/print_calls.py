import falconre
import sys

def print_calls(function):
    for block in function.blocks():
        for instruction in block.instructions():
            operation = instruction.operation()
            if operation.type() == "call":
                print("0x{:08x}: {}".format(instruction.address(), operation))

elf = falconre.falcon.loader.Elf(sys.argv[1])

# If functions you need aren't lifting, you can add them here manually.
# elf.add_user_function(0x12345677)

program = elf.program_recursive()

for function in program.functions():
    raptor_function = falconre.raptor.translate_function(elf, function)
    print_calls(raptor_function)