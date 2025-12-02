#!/bin/bash

# Gmsh tutorial files mesh generator

# Check if gmsh is installed
if ! command -v gmsh &> /dev/null; then
    echo "Error: gmsh is not installed or not in PATH"
    exit 1
fi

# Process each .geo file from t1 to t21
for i in {1..21}; do
    GEO_FILE="t${i}.geo"
    MSH_FILE="t${i}.msh"

    if [ -f "$GEO_FILE" ]; then
        echo "Processing $GEO_FILE..."
        gmsh -3 "$GEO_FILE" -o "$MSH_FILE" -format msh41

        if [ $? -eq 0 ]; then
            echo "  ✓ Generated $MSH_FILE"
        else
            echo "  ✗ Failed to generate $MSH_FILE"
        fi
    else
        echo "  ⊘ Skipping $GEO_FILE (file not found)"
    fi
done

echo ""
echo "Mesh generation complete!"
