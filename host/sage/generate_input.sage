import json
import os
import hashlib
from datetime import datetime
from random import (randbytes, shuffle)

DIGEST_BYTES = 32  # bytes for SHA-256
BLINDER_BYTES = 32
SALT_BYTES = 32
DUPLICATE_COUNT = 2
MERKLE_LEAVES_COUNT = 4

class MerkleTree:
    def __init__(self, leaves):
        self.leaves = leaves
        self.tree = self.build_tree(leaves)
    
    def build_tree(self, leaves):
        tree = [leaves]
        current_level = leaves
        while len(current_level) > 1:
            next_level = []
            for i in range(0, len(current_level), 2):
                left = current_level[i]
                right = current_level[i + 1] if i + 1 < len(current_level) else left
                next_level.append(self.hash_pair(left, right))
            tree.append(next_level)
            current_level = next_level
        return tree

    def hash_pair(self, left, right):
        hasher = hashlib.sha256()
        hasher.update(left + right)
        return hasher.digest()

    def get_root(self):
        return self.tree[-1][0] if self.tree else None

    def get_proof(self, leaf):
        try:
            index = self.leaves.index(leaf)
        except ValueError:
            raise ValueError("Leaf not found in the tree")

        branch = []
        level_index = index
        for level in self.tree[:-1]:
            level_length = len(level)
            if level_index % 2 == 0 and level_index + 1 < level_length:
                branch.append(level[level_index + 1])
            elif level_index % 2 == 1:
                branch.append(level[level_index - 1])
            level_index //= 2

        return {"index": index, "branch": [b.hex() for b in branch]}

def hash_bytes(byte_list):
    hasher = hashlib.sha256()
    hasher.update(byte_list)
    return hasher.digest()

# Generate random values for document_hash, blinder, and salts
document_hash = randbytes(DIGEST_BYTES)
blinder = randbytes(BLINDER_BYTES)
salts = [randbytes(SALT_BYTES) for _ in range(DUPLICATE_COUNT)]

# Compute nullifiers
nullifiers = [hash_bytes(salt + blinder + document_hash) for salt in salts]

# Generate additional leaves and shuffle
leaves = nullifiers + [randbytes(DIGEST_BYTES) for _ in range(MERKLE_LEAVES_COUNT - DUPLICATE_COUNT)]
shuffle(leaves)

# Create Merkle tree and get proofs
tree = MerkleTree(leaves)
root = tree.get_root()
proofs = [tree.get_proof(nullifier) for nullifier in nullifiers]

# Determine the directory of the script
script_dir = os.path.dirname(os.path.abspath(__file__))
test_values_dir = os.path.join(script_dir, '..', 'test_values')

# Generate a unique filename based on the current timestamp
timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")

input_path = os.path.join(test_values_dir, f"input_{timestamp}.json")
output_path = os.path.join(test_values_dir, f"expected_journal_{timestamp}.json")

os.makedirs(test_values_dir, exist_ok=True)

input_data = {
    "document_hash": document_hash.hex(),
    "blinder": blinder.hex(),
    "salts": [salt.hex() for salt in salts],
    "merkle_root": root.hex(),
    "merkle_proofs": proofs
}

with open(input_path, "w") as f:
    json.dump(input_data, f, indent=4)

output_data = {
    "blinder_commitment": hash_bytes(blinder).hex(),
    "document_commitment": hash_bytes(document_hash + blinder).hex(),
    "total_duplicates": int(DUPLICATE_COUNT)
}

with open(output_path, "w") as f:
    json.dump(output_data, f, indent=4)

print("Generated input JSON. Path:", input_path)
print("Generated expected journal JSON. Path:", output_path)
