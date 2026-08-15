# PreviousHighLow benchmark (`previous-session high-low` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 73.22M | 0.012 | 84.26M | 0.558 | 40.85× | 47.00× |
| 10,000 | 0.103 | 96.98M | 0.092 | 109.15M | 5.673 | 55.02× | 61.92× |
| 100,000 | 1.021 | 97.90M | 0.915 | 109.27M | 57.982 | 56.76× | 63.36× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.073 | 0.093 | 1.27× |
| 1 | 5 | 0.247 | 0.332 | 1.34× |
| 1 | 10 | 0.377 | 0.689 | 1.83× |
| 10 | 1 | 0.045 | 0.076 | 1.69× |
| 10 | 5 | 0.180 | 0.359 | 1.99× |
| 10 | 10 | 0.386 | 0.732 | 1.90× |
| 100 | 1 | 0.046 | 0.123 | 2.67× |
| 100 | 5 | 0.194 | 0.597 | 3.08× |
| 100 | 10 | 0.439 | 1.286 | 2.93× |
| 1,000 | 1 | 0.054 | 0.654 | 12.18× |
| 1,000 | 5 | 0.203 | 3.315 | 16.35× |
| 1,000 | 10 | 0.409 | 6.708 | 16.40× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
