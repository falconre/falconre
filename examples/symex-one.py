from log import *
import falconre
import itertools
import sys
import time

il = falconre.falcon.il

info("Analyzing {}".format(sys.argv[1]))

# Let's open our binary
elf = falconre.falcon.loader.Elf(sys.argv[1])

# We need to create a memory model for finch. We'll get this from the elf loader
memory = falconre.finch.executor.Memory.new_with_backing(
    elf.architecture().endian(),
    elf.memory()
)

# Let's set up a dummy stack
STACK_BASE = 0xb000_0000_0000
STACK_SIZE = 0x1000
CONSTANT_NULL_BYTE = il.Constant(0, 8)

for i in range(STACK_SIZE):
    memory.store(STACK_BASE + i, CONSTANT_NULL_BYTE.e())

state = falconre.finch.executor.State(memory)

# We probably want rsp to point into our stack
state.set_scalar("rsp", il.Constant(STACK_BASE, 64).e())

# Let's create some symbolic variables
address, symbolic_variables = state.make_symbolic_buffer("inputs", 8)

# Let's make sure rdi points to our symbolic variables
state.set_scalar("rdi", il.Constant(address, 64).e())

# And now we need a driver. The driver, "Drives," our symbolic state over the
# program.

program = elf.program_recursive()
function = program.function_by_name("run")
program_location = falconre.falcon.il.ProgramLocation.from_function(function)

driver = falconre.finch.executor.Driver(
    program,
    program_location,
    state,
    elf.architecture()
)

STOP_ADDRESS = 0x12b6
TARGET_ADDRESS = 0x11d0

drivers = [driver]

while len(drivers) > 0:
    drivers = itertools.chain.from_iterable(
        map(lambda d: d.step(), drivers)
    )

    drivers = list(filter(lambda d: d.address() != STOP_ADDRESS, drivers))

    for i in range(len(drivers)):
        driver = drivers[i]
        if driver.address() == TARGET_ADDRESS:
            success("Target driver found!")
            for variable in symbolic_variables:
                blue("{}={}".format(
                    variable,
                    driver.state().symbolize_and_eval(variable)
                ))
    
    drivers = list(filter(lambda d: d.address() != TARGET_ADDRESS, drivers))