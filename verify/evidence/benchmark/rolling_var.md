# RollingVariance benchmark (`VAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 148.11M | 0.005 | 183.67M | 0.036 | 5.33× | 6.61× |
| 10,000 | 0.040 | 250.84M | 0.036 | 280.17M | 0.054 | 1.35× | 1.51× |
| 100,000 | 0.350 | 285.53M | 0.334 | 299.27M | 0.239 | 0.68× | 0.71× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.120 | 0.118 | 0.99× |
| 1 | 5 | 0.352 | 0.492 | 1.40× |
| 1 | 10 | 0.472 | 0.998 | 2.12× |
| 10 | 1 | 0.063 | 0.094 | 1.49× |
| 10 | 5 | 0.227 | 0.452 | 1.99× |
| 10 | 10 | 0.472 | 0.929 | 1.97× |
| 100 | 1 | 0.050 | 0.091 | 1.82× |
| 100 | 5 | 0.244 | 0.456 | 1.87× |
| 100 | 10 | 0.461 | 0.920 | 2.00× |
| 1,000 | 1 | 0.052 | 0.103 | 2.00× |
| 1,000 | 5 | 0.236 | 0.482 | 2.04× |
| 1,000 | 10 | 0.581 | 1.001 | 1.72× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
