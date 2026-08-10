# MathSubtract benchmark (`SUB` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 164.50M | 0.004 | 228.70M | 0.034 | 5.53× | 7.69× |
| 10,000 | 0.012 | 838.99M | 0.008 | 1.25G | 0.046 | 3.85× | 5.72× |
| 100,000 | 0.076 | 1.32G | 0.047 | 2.11G | 0.079 | 1.04× | 1.66× |
| 1,000,000 | 2.268 | 441.00M | 1.409 | 709.86M | 1.409 | 0.62× | 1.00× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.063 | 0.109 | 1.72× |
| 1 | 5 | 0.377 | 0.460 | 1.22× |
| 1 | 10 | 0.511 | 1.045 | 2.04× |
| 10 | 1 | 0.057 | 0.097 | 1.71× |
| 10 | 5 | 0.254 | 0.525 | 2.07× |
| 10 | 10 | 0.525 | 1.010 | 1.92× |
| 100 | 1 | 0.070 | 0.120 | 1.70× |
| 100 | 5 | 0.278 | 0.535 | 1.93× |
| 100 | 10 | 0.578 | 0.979 | 1.69× |
| 1,000 | 1 | 0.059 | 0.095 | 1.60× |
| 1,000 | 5 | 0.291 | 0.506 | 1.74× |
| 1,000 | 10 | 0.594 | 1.074 | 1.81× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
