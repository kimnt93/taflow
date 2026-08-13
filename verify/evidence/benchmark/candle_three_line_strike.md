# CandleThreeLineStrike benchmark (`CDL3LINESTRIKE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.105 | 9.52M | 0.094 | 10.66M | 0.032 | 0.30× | 0.34× |
| 10,000 | 0.770 | 12.98M | 0.792 | 12.63M | 0.103 | 0.13× | 0.13× |
| 100,000 | 7.506 | 13.32M | 7.857 | 12.73M | 0.758 | 0.10× | 0.10× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.170 | 0.143 | 0.84× |
| 1 | 5 | 0.427 | 0.542 | 1.27× |
| 1 | 10 | 0.640 | 0.880 | 1.38× |
| 10 | 1 | 0.072 | 0.087 | 1.22× |
| 10 | 5 | 0.321 | 0.439 | 1.37× |
| 10 | 10 | 0.662 | 0.941 | 1.42× |
| 100 | 1 | 0.102 | 0.100 | 0.98× |
| 100 | 5 | 0.354 | 0.456 | 1.29× |
| 100 | 10 | 0.670 | 0.921 | 1.37× |
| 1,000 | 1 | 0.162 | 0.106 | 0.65× |
| 1,000 | 5 | 0.325 | 0.468 | 1.44× |
| 1,000 | 10 | 0.735 | 1.010 | 1.37× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
