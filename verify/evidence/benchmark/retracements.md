# Retracements benchmark (`causal swing retracements` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.254 | 3.94M | 0.245 | 4.08M | 4.750 | 18.70× | 19.39× |
| 10,000 | 2.325 | 4.30M | 2.344 | 4.27M | 46.771 | 20.11× | 19.95× |
| 100,000 | 22.458 | 4.45M | 22.698 | 4.41M | 481.857 | 21.46× | 21.23× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.147 | 0.130 | 0.89× |
| 1 | 5 | 0.447 | 0.504 | 1.13× |
| 1 | 10 | 0.679 | 0.965 | 1.42× |
| 10 | 1 | 0.078 | 0.101 | 1.30× |
| 10 | 5 | 0.345 | 0.494 | 1.43× |
| 10 | 10 | 0.739 | 0.968 | 1.31× |
| 100 | 1 | 0.102 | 0.565 | 5.53× |
| 100 | 5 | 0.331 | 2.715 | 8.21× |
| 100 | 10 | 0.697 | 5.371 | 7.71× |
| 1,000 | 1 | 0.324 | 4.978 | 15.34× |
| 1,000 | 5 | 0.638 | 26.094 | 40.88× |
| 1,000 | 10 | 1.157 | 76.193 | 65.88× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
