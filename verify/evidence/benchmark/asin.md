# MathAsin benchmark (`ASIN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 109.64M | 0.009 | 116.57M | 0.036 | 3.93× | 4.18× |
| 10,000 | 0.074 | 135.61M | 0.067 | 148.49M | 0.098 | 1.33× | 1.45× |
| 100,000 | 0.708 | 141.19M | 0.691 | 144.73M | 0.715 | 1.01× | 1.03× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.078 | 0.130 | 1.66× |
| 1 | 5 | 0.338 | 0.436 | 1.29× |
| 1 | 10 | 0.452 | 0.875 | 1.93× |
| 10 | 1 | 0.046 | 0.088 | 1.90× |
| 10 | 5 | 0.239 | 0.479 | 2.01× |
| 10 | 10 | 0.486 | 0.920 | 1.89× |
| 100 | 1 | 0.051 | 0.086 | 1.69× |
| 100 | 5 | 0.247 | 0.439 | 1.77× |
| 100 | 10 | 0.555 | 0.935 | 1.68× |
| 1,000 | 1 | 0.059 | 0.101 | 1.73× |
| 1,000 | 5 | 0.230 | 0.472 | 2.05× |
| 1,000 | 10 | 0.530 | 1.113 | 2.10× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
