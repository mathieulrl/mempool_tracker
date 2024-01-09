import hashlib
import base58

# Example public key
public_key = bytes.fromhex("036b4a0e8e5040dbdf8c2b0f2e1e9593e9c8f2c9510d1e8a9d57f4d7f9c6f193f1")

# Hash the public key
hashed_public_key = hashlib.new('ripemd160', hashlib.sha256(public_key).digest()).digest()

# Add version byte for testnet (0x6f)
version_byte = b'\x6f'
hashed_public_key_with_version = version_byte + hashed_public_key

# Encode the hashed public key with version byte using Base58Check
address = base58.b58encode_check(hashed_public_key_with_version).decode()

# Construct the wpkh descriptor
descriptor = f"wpkh({address})"

print("Address:", address)
print("Descriptor:", descriptor)