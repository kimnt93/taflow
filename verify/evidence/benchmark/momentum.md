# Momentum benchmark (`MOM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 294.68M | 0.002 | 421.20M | 0.030 | 8.90× | 12.72× |
| 10,000 | 0.019 | 532.38M | 0.017 | 597.39M | 0.034 | 1.79× | 2.01× |
| 100,000 | 0.183 | 547.04M | 0.149 | 670.95M | 0.062 | 0.34× | 0.42× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.085 | 0.108 | 1.27× |
| 1 | 5 | 0.200 | 0.511 | 2.55× |
| 1 | 10 | 0.402 | 1.029 | 2.56× |
| 10 | 1 | 0.048 | 0.096 | 2.00× |
| 10 | 5 | 0.198 | 0.440 | 2.22× |
| 10 | 10 | 0.400 | 0.949 | 2.37× |
| 100 | 1 | 0.048 | 0.115 | 2.39× |
| 100 | 5 | 0.204 | 0.438 | 2.14× |
| 100 | 10 | 0.400 | 0.970 | 2.42× |
| 1,000 | 1 | 0.042 | 0.087 | 2.05× |
| 1,000 | 5 | 0.212 | 0.436 | 2.05× |
| 1,000 | 10 | 0.420 | 0.944 | 2.25× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
