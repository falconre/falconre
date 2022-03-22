import base64
import falconre

falcon = falconre.falcon
il = falcon.il


def il_constant():
    constant = il.Constant(0xdeadbeef, 32)
    assert constant.bits == 32
    assert constant.value_u64 == 0xdeadbeef

def il_scalar():
    scalar = il.Scalar("rax", 64)
    assert scalar.name == "rax"
    assert scalar.bits == 64

def gen_il_expressions():
    return [
        il.Constant(0x1234, 32).e,
        il.Scalar("test", 32).e,
        il.Scalar("lhs", 32).e.add(il.Scalar("rhs", 32).e),
        il.Scalar("lhs", 32).e.sub(il.Scalar("rhs", 32).e),
        il.Scalar("lhs", 32).e.mul(il.Scalar("rhs", 32).e),
        il.Scalar("lhs", 32).e.divu(il.Scalar("rhs", 32).e),
        il.Scalar("lhs", 32).e.modu(il.Scalar("rhs", 32).e),
        il.Scalar("lhs", 32).e.divs(il.Scalar("rhs", 32).e),
        il.Scalar("lhs", 32).e.mods(il.Scalar("rhs", 32).e),
        il.Scalar("lhs", 32).e.and_(il.Scalar("rhs", 32).e),
        il.Scalar("lhs", 32).e.or_(il.Scalar("rhs", 32).e),
        il.Scalar("lhs", 32).e.xor_(il.Scalar("rhs", 32).e),
        il.Scalar("lhs", 32).e.shl(il.Scalar("rhs", 32).e),
        il.Scalar("lhs", 32).e.shr(il.Scalar("rhs", 32).e),
        il.Scalar("lhs", 32).e.ashr(il.Scalar("rhs", 32).e),
        il.Scalar("lhs", 32).e.cmpeq(il.Scalar("rhs", 32).e),
        il.Scalar("lhs", 32).e.cmpneq(il.Scalar("rhs", 32).e),
        il.Scalar("lhs", 32).e.cmpltu(il.Scalar("rhs", 32).e),
        il.Scalar("lhs", 32).e.cmplts(il.Scalar("rhs", 32).e),
        il.Scalar("lhs", 32).e.cmpeq(il.Scalar("rhs", 32).e).ite(
            il.Constant(1, 32).e,
            il.Constant(0, 32).e
        )
    ]

def gen_il_operations():
    return [
        il.Operation.
        il.Operation.load(il.Scalar("dest", 32), il.Constant(0xdeadbeef, 32)),

    ]

il_constant()
il_scalar()
gen_il_expressions()

memory_backing = falcon.memory.Memory(falcon.architecture.Endian.little())
memory_backing.set_memory(
    0,
    base64.b16decode("48b8efbeadde0000000048bbefbeadde000000004889c14801d9".upper()),
    falcon.memory.MemoryPermissions(7)
)

amd64 = falcon.architecture.Architecture.amd64()
function = amd64.translate_function(memory_backing, 0)

print(function)

for block in function.blocks:
    for instruction in block.instructions:
        print(instruction)