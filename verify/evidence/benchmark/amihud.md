# Amihud benchmark (`AmihudIlliquidity` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 86.11M | 0.010 | 97.83M | 0.503 | 43.29× | 49.17× |
| 10,000 | 0.072 | 138.71M | 0.069 | 144.33M | 3.847 | 53.37× | 55.53× |
| 100,000 | 0.665 | 150.30M | 0.634 | 157.78M | 37.908 | 56.97× | 59.81× |
| 1,000,000 | 6.802 | 147.01M | 6.391 | 156.48M | 384.002 | 56.45× | 60.09× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.090 | 0.245 | 2.72× |
| 1 | 5 | 0.382 | 1.053 | 2.76× |
| 1 | 10 | 0.478 | 2.343 | 4.90× |
| 10 | 1 | 0.059 | 0.225 | 3.81× |
| 10 | 5 | 0.233 | 1.054 | 4.51× |
| 10 | 10 | 0.495 | 2.422 | 4.89× |
| 100 | 1 | 0.051 | 0.247 | 4.81× |
| 100 | 5 | 0.230 | 1.228 | 5.33× |
| 100 | 10 | 0.479 | 2.840 | 5.93× |
| 1,000 | 1 | 0.067 | 0.604 | 9.04× |
| 1,000 | 5 | 0.288 | 3.402 | 11.82× |
| 1,000 | 10 | 0.525 | 6.503 | 12.38× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
