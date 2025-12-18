#!/bin/bash
# Test script for EngineIdentify

# 1. Start the server in background if not running (assumes user runs it separately or we run it)
# We will just use curl to test if it's running, if not we assume user will run it.
# Actually, I'll just write the curl command corresponding to the ambiguity test.

# Ambiguity Test:
# File 1: resources/app.asar (Strong Electron signal, weight 2.0)
# File 2: data.win (Strong GameMaker signal, weight 2.0)
# File 3: options.ini (Weak GameMaker signal, weight 0.5)
# Expected Result: GameMaker (2.5) > Electron (2.0)

echo "Testing Ambiguity: GameMaker vs Electron..."
curl -X POST http://localhost:3000/identify \
  -H "Content-Type: application/json" \
  -d '{
    "files": [
        "resources/app.asar", 
        "data.win",
        "options.ini"
    ]
  }'
echo "\n"

# Test 2: Unity vs Godot
# File 1: game.pck (Godot, 2.0)
# File 2: assets (Unity, 1.5)
# Expected: Godot
echo "Testing Ambiguity: Godot vs Unity..."
curl -X POST http://localhost:3000/identify \
  -H "Content-Type: application/json" \
  -d '{
    "files": [
        "game.pck", 
        "shared.assets"
    ]
  }'
echo "\n"
