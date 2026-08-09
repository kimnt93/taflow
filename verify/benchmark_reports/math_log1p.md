# MathLog1p benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 78.49M | 0.011 | 94.14M | 0.010 | 0.79× | 0.95× |
| 10,000 | 0.101 | 98.58M | 0.097 | 102.72M | 0.085 | 0.84× | 0.87× |
| 100,000 | 0.935 | 106.92M | 0.918 | 108.88M | 0.807 | 0.86× | 0.88× |
| 1,000,000 | 10.095 | 99.06M | 10.359 | 96.54M | 8.279 | 0.82× | 0.80× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.097 | 0.065 | 0.67× |
| 1 | 5 | 0.283 | 0.211 | 0.75× |
| 1 | 10 | 0.488 | 0.391 | 0.80× |
| 10 | 1 | 0.052 | 0.044 | 0.85× |
| 10 | 5 | 0.240 | 0.208 | 0.87× |
| 10 | 10 | 0.556 | 0.445 | 0.80× |
| 100 | 1 | 0.052 | 0.045 | 0.86× |
| 100 | 5 | 0.225 | 0.212 | 0.94× |
| 100 | 10 | 0.483 | 0.423 | 0.88× |
| 1,000 | 1 | 0.059 | 0.064 | 1.08× |
| 1,000 | 5 | 0.263 | 0.219 | 0.83× |
| 1,000 | 10 | 0.518 | 0.478 | 0.92× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
