# HeikinAshi benchmark (`HeikinAshi` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 71.04M | 0.012 | 82.63M | 0.580 | 41.17× | 47.88× |
| 10,000 | 0.083 | 120.08M | 0.075 | 134.00M | 4.471 | 53.69× | 59.91× |
| 100,000 | 0.752 | 133.04M | 0.685 | 146.01M | 50.171 | 66.75× | 73.26× |
| 1,000,000 | 25.393 | 39.38M | 7.294 | 137.09M | 546.578 | 21.53× | 74.93× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.085 | 0.224 | 2.64× |
| 1 | 5 | 0.343 | 0.878 | 2.56× |
| 1 | 10 | 0.495 | 13.077 | 26.44× |
| 10 | 1 | 0.054 | 0.172 | 3.22× |
| 10 | 5 | 0.230 | 0.900 | 3.91× |
| 10 | 10 | 0.502 | 2.024 | 4.03× |
| 100 | 1 | 0.053 | 0.229 | 4.29× |
| 100 | 5 | 0.240 | 1.458 | 6.07× |
| 100 | 10 | 0.513 | 2.621 | 5.11× |
| 1,000 | 1 | 0.068 | 0.792 | 11.58× |
| 1,000 | 5 | 0.262 | 3.684 | 14.05× |
| 1,000 | 10 | 0.528 | 7.380 | 13.98× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
