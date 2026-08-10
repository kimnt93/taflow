# LogReturn benchmark (`LogReturn` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 103.52M | 0.009 | 108.99M | 0.163 | 16.87× | 17.76× |
| 10,000 | 0.078 | 127.83M | 0.075 | 133.65M | 0.541 | 6.91× | 7.22× |
| 100,000 | 0.745 | 134.17M | 0.722 | 138.55M | 4.297 | 5.76× | 5.95× |
| 1,000,000 | 7.889 | 126.76M | 7.171 | 139.46M | 41.759 | 5.29× | 5.82× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.072 | 0.266 | 3.67× |
| 1 | 5 | 0.273 | 1.344 | 4.92× |
| 1 | 10 | 0.487 | 2.248 | 4.61× |
| 10 | 1 | 0.053 | 0.228 | 4.34× |
| 10 | 5 | 0.232 | 1.304 | 5.63× |
| 10 | 10 | 0.484 | 2.702 | 5.58× |
| 100 | 1 | 0.052 | 0.220 | 4.20× |
| 100 | 5 | 0.305 | 1.324 | 4.34× |
| 100 | 10 | 0.490 | 2.340 | 4.78× |
| 1,000 | 1 | 0.060 | 0.268 | 4.49× |
| 1,000 | 5 | 0.245 | 1.485 | 6.06× |
| 1,000 | 10 | 0.525 | 2.658 | 5.07× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
