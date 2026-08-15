# FastStochasticOscillator benchmark (`STOCHF` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 70.55M | 0.011 | 88.40M | 0.045 | 3.16× | 3.96× |
| 10,000 | 0.108 | 92.20M | 0.104 | 96.41M | 0.136 | 1.26× | 1.31× |
| 100,000 | 1.568 | 63.77M | 1.022 | 97.86M | 1.299 | 0.83× | 1.27× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.080 | 0.152 | 1.89× |
| 1 | 5 | 0.301 | 0.551 | 1.83× |
| 1 | 10 | 0.413 | 1.013 | 2.45× |
| 10 | 1 | 0.048 | 0.095 | 2.00× |
| 10 | 5 | 0.200 | 0.490 | 2.45× |
| 10 | 10 | 0.409 | 1.009 | 2.47× |
| 100 | 1 | 0.045 | 0.108 | 2.40× |
| 100 | 5 | 0.195 | 0.485 | 2.49× |
| 100 | 10 | 0.421 | 1.059 | 2.52× |
| 1,000 | 1 | 0.059 | 0.115 | 1.96× |
| 1,000 | 5 | 0.202 | 0.571 | 2.83× |
| 1,000 | 10 | 0.448 | 1.115 | 2.49× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
