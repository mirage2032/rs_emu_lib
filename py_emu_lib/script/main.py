import py_emu_lib
from typing import List


class MD(py_emu_lib.memory.MemDevice):
    def __init__(self, data: List[int]):
        self.data = data
        self.set_cb_size(lambda: self.size())
        self.set_cb_read(lambda addr: self.read(addr))
        self.set_cb_write(lambda addr, data: self.write(addr, data))
        self.set_cb_ro(lambda: self.is_read_only())
        self.set_cb_clear(lambda: self.clear())

    def size(self):
        return len(self.data)

    def read(self, addr):
        return self.data[addr]

    def write(self, addr, data):
        self.data[addr] = data

    def is_read_only(self):
        return False

    def clear(self):
        self.data = [0] * len(self.data)


def main():
    memdev = MD([1, 2, 3, 4, 5, 6, 7, 8, 9, 10])
    memdev2 = MD([10, 9, 8, 7, 6, 5, 4, 3, 2, 1])
    memory = py_emu_lib.memory.Memory().empty()
    memory.add_device(memdev)
    memory.add_device(memdev2)
    print()
    for i in range(memory.len()):
        print(memory.read8(i), end=' ')
    memory.save('mem.bin')


if __name__ == '__main__':
    main()
