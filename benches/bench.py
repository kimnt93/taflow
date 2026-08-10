#!/usr/bin/env python3
"""Compatibility launcher for the canonical verification benchmark.

The implementation lives in ``scripts/verification/benchmark.py`` so
correctness and performance use one TA-Lib/Wickra registry.
"""

from __future__ import annotations

import runpy
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SCRIPTS = ROOT / "scripts" / "verification"
sys.path.insert(0, str(SCRIPTS))
runpy.run_path(str(SCRIPTS / "benchmark.py"), run_name="__main__")
