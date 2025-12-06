#!/bin/bash

# Check if gmsh is installed
if ! command -v gmsh &> /dev/null; then
    echo "Error: gmsh is not installed or not in PATH"
    exit 1
fi

# Function to generate mesh
# Usage: generate_mesh <geo_filename> [gmsh_options...]
generate_mesh() {
    local input_geo="$1"
    shift # Remove the first argument (filename), use the rest as options
    local options="$@"
    
    # Automatically determine output filename (.geo -> .msh)
    local output_msh="${input_geo%.geo}.msh"

    echo "Generating $output_msh from $input_geo..."
    
    # Run Gmsh
    # Note: We don't quote $options here to allow multiple flags to be expanded correctly
    gmsh "$input_geo" -o "$output_msh" $options
    
    if [ $? -eq 0 ]; then
        echo "[✓] Successfully created $output_msh"
    else
        echo "[✗] Error generating $output_msh"
        exit 1
    fi
    echo "----------------------------------------"
}

# --- Mesh Generation List ---

# 1. Box (3D, Partitioned with Ghost Cells)
generate_mesh "box.geo" -3 -part 2 -setnumber Mesh.PartitionCreateGhostCells 1

# Add more files here as needed:
# generate_mesh "example.geo" -3
