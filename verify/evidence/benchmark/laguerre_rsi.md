# LaguerreRelativeStrengthIndex benchmark (`LaguerreRSI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 76.34M | 0.011 | 93.92M | 0.201 | 15.36× | 18.90× |
| 10,000 | 0.104 | 95.86M | 0.086 | 115.93M | 0.589 | 5.65× | 6.83× |
| 100,000 | 1.019 | 98.09M | 0.823 | 121.57M | 4.811 | 4.72× | 5.85× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.070 | 0.348 | 4.99× |
| 1 | 5 | 0.285 | 1.187 | 4.16× |
| 1 | 10 | 0.501 | 2.516 | 5.03× |
| 10 | 1 | 0.052 | 0.227 | 4.39× |
| 10 | 5 | 0.221 | 1.362 | 6.17× |
| 10 | 10 | 0.457 | 2.578 | 5.64× |
| 100 | 1 | 0.050 | 0.218 | 4.33× |
| 100 | 5 | 0.233 | 1.449 | 6.22× |
| 100 | 10 | 0.529 | 2.458 | 4.65× |
| 1,000 | 1 | 0.059 | 0.278 | 4.71× |
| 1,000 | 5 | 0.247 | 1.660 | 6.72× |
| 1,000 | 10 | 0.510 | 2.927 | 5.74× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
