import falconre
import sys
import time

elf = falconre.falcon.loader.Elf(sys.argv[1])
program = elf.program_recursive()

function = \
    falconre.raptor.translate_function(
        elf,
        program.function_by_name(sys.argv[2])
    )

# strided_intervals = falconre.raptor.analysis.strided_intervals(function)
stack_buffers = falconre.raptor.analysis.stack_buffers(function)

for block in function.blocks():
    print("block: 0x{:x}".format(block.index()))
    for instruction in block.instructions():
        print(instruction)
        pl = falconre.raptor.ir.ProgramLocation(
            function.index(),
            falconre.raptor.ir.FunctionLocation.from_instruction(
                block.index(),
                instruction.index()
            )
        )
        rbx = falconre.raptor.ir.Scalar("rbx", 64).variable()
        if not instruction.variables():
            continue
        for var in instruction.variables():
            value = stack_buffers[pl].eval(var.e())
            if value:
                print("    ", var, value)
    print()

for edge in function.edges():
    print(edge)