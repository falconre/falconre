from log import *
import falconre
import sys
import time

print("Obsolete until raptor comes back")
sys.exit(1)



def filter_stack_variables(variables):
    stack_variables = []
    for variable in variables:
        if variable.stack_variable():
            stack_variables.append(variable.stack_variable())
    return stack_variables


def analysis(function):
    strided_intervals = None

    # Collect all of the stack variables
    stack_variables = []
    for block in function.blocks():
        for instruction in block.instructions():
            variables = instruction.variables()
            if variables == None:
                continue
            for variable in variables:
                stack_variable = variable.stack_variable()
                if not stack_variable:
                    continue
                if stack_variable not in stack_variables:
                    stack_variables.append(stack_variable)
 
    function_stack_variables = list(set(stack_variables))

    # For every instruction
    for block in function.blocks():
        for instruction in block.instructions():

            # See if we are storing/writing to the stack
            if instruction.operation().type() != "store":
                continue

            # Index is the address we are writing to
            index = instruction.operation().index()

            # Make sure we have exactly one stack variable in the address
            stack_variables = filter_stack_variables(index.variables())
            if len(stack_variables) != 1:
                continue
            stack_variable = stack_variables[0]

            alert("Found interesting stack write: {}".format(instruction))

            # Can we isolate that stack_variable?
            expression = None
            if not index.lhs().stack_pointer():
                expression = index.lhs()
            elif not index.rhs().stack_pointer():
                expression = index.rhs()
            else:
                error("Could not isolate non-stack variable part of expression")
                error(index.lhs())
                error(index.rhs())
                continue

            # We need to compute strided intervals for this function
            if strided_intervals == None:
                strided_intervals = falconre.raptor.analysis.strided_intervals(function)

            program_location = falconre.raptor.ir.ProgramLocation(
                function.index(),
                falconre.raptor.ir.FunctionLocation.from_instruction(
                    block.index(),
                    instruction.index()
                )
            )

            si = strided_intervals[program_location].eval(expression)
            blue("{}, {}".format(expression, si))
            hi = si.hi()
            if not hi:
                alert("[!] could not get upper bounds for {}, {}".format(expression, si))
            else:
                max_stack_value = stack_variable.offset() + hi.value_u64()
                found_overflow = False
                for sv in function_stack_variables:
                    if sv == stack_variable:
                        continue
                    if sv.offset() > stack_variable.offset() and sv.offset() <= max_stack_value:
                        found_overflow = True
                        success("[+] Potential overflow against {}!".format(sv))
                if not found_overflow:
                    alert("No overflow found")


info("Analyzing {}".format(sys.argv[1]))
elf = falconre.falcon.loader.Elf(sys.argv[1])

start_time = time.time()
program = elf.program_recursive()
lifted_time = time.time()
program = falconre.raptor.translate_program_parallel(elf, program)
translated_time = time.time()

info("Lifted in {}s".format(lifted_time - start_time))
info("Translated in {}s".format(translated_time - lifted_time))

for function in program.functions():
    info("{:X}: {}".format(function.address(), function.name()))
    analysis(function)

info("Analysis completed in {}s".format(time.time() - translated_time))