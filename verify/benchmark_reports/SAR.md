# ParabolicSar benchmark (`SAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 85.90M | 0.011 | 90.77M | 0.040 | 3.46× | 3.65× |
| 10,000 | 0.113 | 88.23M | 0.126 | 79.35M | 0.097 | 0.86× | 0.77× |
| 100,000 | 1.141 | 87.65M | 1.106 | 90.38M | 0.656 | 0.57× | 0.59× |
| 1,000,000 | 11.591 | 86.27M | 11.223 | 89.10M | 6.225 | 0.54× | 0.55× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.120 | 0.168 | 1.40× |
| 1 | 5 | 0.254 | 0.517 | 2.04× |
| 1 | 10 | 0.507 | 1.085 | 2.14× |
| 10 | 1 | 0.049 | 0.098 | 1.99× |
| 10 | 5 | 0.213 | 0.497 | 2.33× |
| 10 | 10 | 0.479 | 1.051 | 2.19× |
| 100 | 1 | 0.053 | 0.100 | 1.88× |
| 100 | 5 | 0.255 | 0.507 | 1.99× |
| 100 | 10 | 0.487 | 1.041 | 2.14× |
| 1,000 | 1 | 0.064 | 0.126 | 1.99× |
| 1,000 | 5 | 0.244 | 0.519 | 2.12× |
| 1,000 | 10 | 0.507 | 1.082 | 2.13× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
