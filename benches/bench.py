#!/usr/bin/env python3
"""Compatibility launcher for the canonical verification benchmark.

The implementation lives in ``verify/benchmark.py`` so correctness and
performance use one CHECK.md-derived TA-Lib-to-taflow registry.
"""

from __future__ import annotations

import runpy
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
VERIFY = ROOT / "verify"
sys.path.insert(0, str(VERIFY))
runpy.run_path(str(VERIFY / "benchmark.py"), run_name="__main__")
