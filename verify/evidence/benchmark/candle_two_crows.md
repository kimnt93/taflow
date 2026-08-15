# CandleTwoCrows benchmark (`CDL2CROWS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 167.53M | 0.003 | 377.67M | 0.031 | 5.28× | 11.89× |
| 10,000 | 0.071 | 140.26M | 0.066 | 151.81M | 0.110 | 1.54× | 1.67× |
| 100,000 | 0.756 | 132.35M | 0.773 | 129.32M | 0.860 | 1.14× | 1.11× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.131 | 0.104 | 0.80× |
| 1 | 5 | 0.244 | 0.477 | 1.96× |
| 1 | 10 | 0.422 | 0.927 | 2.19× |
| 10 | 1 | 0.045 | 0.086 | 1.89× |
| 10 | 5 | 0.184 | 0.429 | 2.33× |
| 10 | 10 | 0.385 | 0.960 | 2.49× |
| 100 | 1 | 0.044 | 0.089 | 2.02× |
| 100 | 5 | 0.185 | 0.439 | 2.37× |
| 100 | 10 | 0.391 | 0.898 | 2.30× |
| 1,000 | 1 | 0.053 | 0.096 | 1.82× |
| 1,000 | 5 | 0.217 | 0.506 | 2.33× |
| 1,000 | 10 | 0.432 | 0.959 | 2.22× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
