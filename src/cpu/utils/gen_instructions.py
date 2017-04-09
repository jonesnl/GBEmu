import itertools

start_val = int(input("Opcode start val: "), 16)
opcode_counter = itertools.count(start_val) 
opcode = 0x00

with open('outfile.rs', 'w') as f:
    while opcode <= 0xff:
        opcode = next(opcode_counter)
        print("Next opcode: {:#04x}".format(opcode))

        if opcode % 8 == 0:
            func = input("Function: ")

        cycles = "16" if opcode % 8 == 6 else "8"

        f.write("Instruction {\n")
        f.write("    opcode: {:#04x},\n".format(opcode))
        f.write("    func: {},\n".format(func))
        f.write("    cycles: {},\n".format(cycles))
        f.write("},\n")
