import yaml
import json
import sys

# Read YAML schema
with open(sys.argv[1], 'r') as f:
    schema = yaml.safe_load(f)

# Write JSON
with open(sys.argv[2], 'w') as f:
    json.dump(schema, f, indent=2)

print(f"Converted {sys.argv[1]} to {sys.argv[2]}")
