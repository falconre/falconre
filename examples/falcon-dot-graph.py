import falconre
import sys
import time

elf = falconre.falcon.loader.Elf(sys.argv[1])
program = elf.program_recursive()

print(program.function_by_name(sys.argv[2]).control_flow_graph.dot_graph)