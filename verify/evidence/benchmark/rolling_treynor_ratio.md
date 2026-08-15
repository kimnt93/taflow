# RollingTreynorRatio benchmark (`TreynorRatio` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.024 | 42.10M | 0.023 | 44.13M | 0.224 | 9.42× | 9.88× |
| 10,000 | 0.220 | 45.46M | 0.220 | 45.48M | 0.929 | 4.22× | 4.23× |
| 100,000 | 2.177 | 45.95M | 2.125 | 47.07M | 8.157 | 3.75× | 3.84× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.057 | 0.290 | 5.08× |
| 1 | 5 | 0.297 | 1.133 | 3.81× |
| 1 | 10 | 0.473 | 2.428 | 5.14× |
| 10 | 1 | 0.053 | 0.229 | 4.28× |
| 10 | 5 | 0.207 | 1.426 | 6.88× |
| 10 | 10 | 0.399 | 2.474 | 6.19× |
| 100 | 1 | 0.049 | 0.243 | 4.95× |
| 100 | 5 | 0.227 | 1.373 | 6.05× |
| 100 | 10 | 0.445 | 2.525 | 5.67× |
| 1,000 | 1 | 0.076 | 0.315 | 4.14× |
| 1,000 | 5 | 0.232 | 1.799 | 7.75× |
| 1,000 | 10 | 0.439 | 3.387 | 7.71× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
