# RollingValueAtRisk benchmark (`ValueAtRisk` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.128 | 7.81M | 0.186 | 5.38M | 0.356 | 2.78× | 1.92× |
| 10,000 | 1.290 | 7.75M | 1.406 | 7.11M | 1.832 | 1.42× | 1.30× |
| 100,000 | 12.793 | 7.82M | 14.944 | 6.69M | 16.031 | 1.25× | 1.07× |
| 1,000,000 | 134.710 | 7.42M | 125.496 | 7.97M | 158.525 | 1.18× | 1.26× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.174 | 0.454 | 2.61× |
| 1 | 5 | 0.371 | 1.240 | 3.34× |
| 1 | 10 | 0.479 | 2.578 | 5.39× |
| 10 | 1 | 0.052 | 0.227 | 4.40× |
| 10 | 5 | 0.229 | 1.125 | 4.92× |
| 10 | 10 | 0.466 | 2.415 | 5.18× |
| 100 | 1 | 0.067 | 0.255 | 3.82× |
| 100 | 5 | 0.239 | 1.448 | 6.06× |
| 100 | 10 | 0.488 | 2.629 | 5.39× |
| 1,000 | 1 | 0.188 | 0.402 | 2.14× |
| 1,000 | 5 | 0.359 | 2.193 | 6.11× |
| 1,000 | 10 | 0.593 | 4.239 | 7.14× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
