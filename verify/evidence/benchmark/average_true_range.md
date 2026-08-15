# AverageTrueRange benchmark (`ATR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 127.86M | 0.006 | 157.38M | 0.040 | 5.12× | 6.31× |
| 10,000 | 0.058 | 172.23M | 0.053 | 189.15M | 0.088 | 1.52× | 1.67× |
| 100,000 | 0.543 | 184.31M | 0.506 | 197.82M | 0.673 | 1.24× | 1.33× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.064 | 0.145 | 2.26× |
| 1 | 5 | 0.285 | 0.476 | 1.67× |
| 1 | 10 | 0.386 | 0.964 | 2.50× |
| 10 | 1 | 0.047 | 0.103 | 2.21× |
| 10 | 5 | 0.185 | 0.509 | 2.76× |
| 10 | 10 | 0.392 | 0.931 | 2.38× |
| 100 | 1 | 0.042 | 0.093 | 2.23× |
| 100 | 5 | 0.180 | 0.484 | 2.68× |
| 100 | 10 | 0.428 | 0.976 | 2.28× |
| 1,000 | 1 | 0.047 | 0.097 | 2.06× |
| 1,000 | 5 | 0.185 | 0.469 | 2.54× |
| 1,000 | 10 | 0.445 | 1.052 | 2.37× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
