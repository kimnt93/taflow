# AverageTrueRange benchmark (`ATR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.059 | 17.01M | 0.053 | 18.79M | 0.037 | 0.63× | 0.70× |
| 10,000 | 0.426 | 23.48M | 0.420 | 23.80M | 0.087 | 0.20× | 0.21× |
| 100,000 | 4.066 | 24.59M | 4.332 | 23.08M | 0.561 | 0.14× | 0.13× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.123 | 0.116 | 0.94× |
| 1 | 5 | 0.479 | 0.485 | 1.01× |
| 1 | 10 | 0.647 | 0.944 | 1.46× |
| 10 | 1 | 0.066 | 0.090 | 1.37× |
| 10 | 5 | 0.304 | 0.444 | 1.46× |
| 10 | 10 | 0.621 | 0.920 | 1.48× |
| 100 | 1 | 0.073 | 0.094 | 1.30× |
| 100 | 5 | 0.309 | 0.457 | 1.48× |
| 100 | 10 | 0.628 | 0.951 | 1.52× |
| 1,000 | 1 | 0.111 | 0.101 | 0.90× |
| 1,000 | 5 | 0.309 | 0.478 | 1.55× |
| 1,000 | 10 | 0.692 | 1.053 | 1.52× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
