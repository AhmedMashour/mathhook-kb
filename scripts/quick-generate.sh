#!/bin/bash
# Quick generate all outputs
mkdir -p outputs
SUCCESS=0
FAILED=0
for f in $(find schemas -name "*.yaml"); do
  if ./target/release/kb-cli build "$f" --output outputs >/dev/null 2>&1; then
    echo "✅ $f"
    SUCCESS=$((SUCCESS + 1))
  else
    echo "❌ $f"
    FAILED=$((FAILED + 1))
  fi
done
echo ""
echo "Done! Success: $SUCCESS, Failed: $FAILED"
