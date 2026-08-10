# ParabolicSar benchmark (`SAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 71.11M | 0.013 | 74.65M | 0.044 | 3.11× | 3.27× |
| 10,000 | 0.125 | 79.73M | 0.119 | 83.91M | 0.110 | 0.88× | 0.93× |
| 100,000 | 1.216 | 82.24M | 1.179 | 84.81M | 0.709 | 0.58× | 0.60× |
| 1,000,000 | 12.620 | 79.24M | 11.913 | 83.94M | 6.720 | 0.53× | 0.56× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.085 | 0.134 | 1.58× |
| 1 | 5 | 0.278 | 0.603 | 2.17× |
| 1 | 10 | 0.559 | 1.684 | 3.01× |
| 10 | 1 | 0.079 | 0.122 | 1.55× |
| 10 | 5 | 0.285 | 0.528 | 1.85× |
| 10 | 10 | 0.553 | 1.236 | 2.23× |
| 100 | 1 | 0.081 | 0.142 | 1.76× |
| 100 | 5 | 0.283 | 0.503 | 1.78× |
| 100 | 10 | 0.563 | 1.132 | 2.01× |
| 1,000 | 1 | 0.083 | 0.115 | 1.38× |
| 1,000 | 5 | 0.310 | 0.609 | 1.96× |
| 1,000 | 10 | 0.544 | 1.113 | 2.05× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
