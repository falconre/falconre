import falconre

elf = falconre.falcon.loader.Elf("/bin/ls")

program = elf.program_recursive()
for function in program.functions():
    print(function)
    for block in function.blocks():
        print(block)