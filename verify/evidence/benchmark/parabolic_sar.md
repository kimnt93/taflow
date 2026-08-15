# ParabolicSar benchmark (`SAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 118.65M | 0.007 | 135.39M | 0.037 | 4.35× | 4.96× |
| 10,000 | 0.077 | 129.21M | 0.073 | 136.72M | 0.092 | 1.19× | 1.26× |
| 100,000 | 0.891 | 112.22M | 0.874 | 114.37M | 0.603 | 0.68× | 0.69× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.173 | 0.155 | 0.89× |
| 1 | 5 | 0.257 | 0.545 | 2.12× |
| 1 | 10 | 0.386 | 0.966 | 2.50× |
| 10 | 1 | 0.048 | 0.097 | 2.01× |
| 10 | 5 | 0.180 | 0.521 | 2.90× |
| 10 | 10 | 0.410 | 1.005 | 2.45× |
| 100 | 1 | 0.041 | 0.098 | 2.37× |
| 100 | 5 | 0.196 | 0.472 | 2.41× |
| 100 | 10 | 0.394 | 0.955 | 2.42× |
| 1,000 | 1 | 0.048 | 0.104 | 2.17× |
| 1,000 | 5 | 0.188 | 0.490 | 2.60× |
| 1,000 | 10 | 0.419 | 1.008 | 2.41× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
