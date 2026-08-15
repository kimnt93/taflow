# CandleThreeInside benchmark (`CDL3INSIDE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 117.27M | 0.005 | 212.55M | 0.040 | 4.63× | 8.40× |
| 10,000 | 0.101 | 99.20M | 0.099 | 101.20M | 0.135 | 1.34× | 1.37× |
| 100,000 | 1.062 | 94.18M | 1.014 | 98.60M | 1.054 | 0.99× | 1.04× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.071 | 0.123 | 1.74× |
| 1 | 5 | 0.329 | 0.438 | 1.33× |
| 1 | 10 | 0.382 | 0.953 | 2.50× |
| 10 | 1 | 0.045 | 0.099 | 2.22× |
| 10 | 5 | 0.202 | 0.445 | 2.20× |
| 10 | 10 | 0.396 | 0.907 | 2.29× |
| 100 | 1 | 0.045 | 0.090 | 2.00× |
| 100 | 5 | 0.208 | 0.506 | 2.43× |
| 100 | 10 | 0.431 | 0.925 | 2.15× |
| 1,000 | 1 | 0.056 | 0.100 | 1.78× |
| 1,000 | 5 | 0.193 | 0.504 | 2.61× |
| 1,000 | 10 | 0.442 | 1.069 | 2.42× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
