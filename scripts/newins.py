#!/usr/bin/python3
# Script used for generating new instructions for the CPU
import os
import sys

CPU_TEMPLATES = {
    "Z80": os.path.join(os.path.dirname(os.path.abspath(__file__)), "data/z80_instruction.rs.template"),
    "I8080": os.path.join(os.path.dirname(os.path.abspath(__file__)), "data/i8080_instruction.rs.template"),
}
def generate_instruction(cpu_name: str, instruction_filename: str, instruction_typename: str):
    if cpu_name in CPU_TEMPLATES.keys():
        try:
            with open(CPU_TEMPLATES[cpu_name], "r") as f:
                template = f.read()
        except Exception as e:
            raise e
        template = template.replace("%INSTRUCTION%", instruction_typename)
        with open(instruction_filename + ".rs", "w") as f:
            f.write(template)
    else:
        raise ValueError(f"CPU {cpu_name} not supported")


if __name__ == "__main__":
    args = sys.argv[1:]
    args[0] = args[0].upper()
    if len(args) not in (2, 3):
        print("Usage: newins.py <cpu_name> <instruction_filename> [<instruction_typename>]\n"
              "If no instruction_typename is provided, the uppercase instruction_filename will be used\n"
              "Example: newins.py Z80 ld_bc_nn\n"
              "Example: newins.py I8080 halt Halt")
        exit(1)
    if len(args) == 2:
        args.append(args[1].upper())
    generate_instruction(*args)
    sys.exit(1)
