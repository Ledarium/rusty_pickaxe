from Crypto.Hash import keccak
from eth_abi.packed import encode_abi_packed

abi_types = ["uint256", "bytes32", "address", "address", "uint", "uint", "uint"]
packed = encode_abi_packed(
            abi_types,
            [1, b'b'*32, "0xFFcf8FDEE72ac11b5c542428B35EEF5769C409f0", "0x90F8bf6A479f320ead074411a4B0e7944Ea8c9C1", 1, 20, 2])
print(', '.join(str(byte) for byte in packed))
print(packed.hex())
k = keccak.new(digest_bits=256)
k.update(packed)
hx = k.hexdigest()
print(hx)
