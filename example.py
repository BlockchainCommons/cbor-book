import cbor2

def to_hex(b: bytes) -> str:
    return ' '.join(f'{byte:02x}' for byte in b)

examples = [
    (0.0, "f9 0000"),
    (1.0, "f9 3c00"),
    (-1.5, "f9 bc00"),
    (1.1, "fa 3f8ccccd"),
    (3.14159, "fa 40490fd0"),
    (1.0e+300, "fb 7e37e43c8800759c"),
    (float('inf'), "f9 7c00"),
    (float('nan'), None),  # NaN payloads vary and should not be strict
]

print(f"{'Value':>12} | {'Encoded Hex':<23} | {'Expected':<23} | Match")
print("-" * 70)

for value, expected in examples:
    encoded = cbor2.dumps(value, canonical=True)
    hex_str = to_hex(encoded)
    match = (expected is None) or (hex_str.lower() == expected.lower())
    print(f"{repr(value):>12} | {hex_str:<23} | {expected or 'n/a':<23} | {match}")
