# Crossover benchmark (`causal crossover` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 131.52M | 0.006 | 161.02M | 0.016 | 2.08× | 2.55× |
| 10,000 | 0.036 | 275.32M | 0.032 | 315.46M | 0.026 | 0.73× | 0.84× |
| 100,000 | 0.287 | 348.74M | 0.267 | 374.11M | 0.134 | 0.47× | 0.50× |
| 1,000,000 | 3.646 | 274.25M | 2.857 | 349.99M | 2.993 | 0.82× | 1.05× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.124 | 0.117 | 0.94× |
| 1 | 5 | 0.331 | 0.325 | 0.98× |
| 1 | 10 | 0.489 | 0.670 | 1.37× |
| 10 | 1 | 0.046 | 0.070 | 1.52× |
| 10 | 5 | 0.218 | 0.313 | 1.43× |
| 10 | 10 | 0.469 | 0.668 | 1.42× |
| 100 | 1 | 0.050 | 0.071 | 1.44× |
| 100 | 5 | 0.222 | 0.340 | 1.53× |
| 100 | 10 | 0.484 | 0.679 | 1.40× |
| 1,000 | 1 | 0.059 | 0.066 | 1.12× |
| 1,000 | 5 | 0.242 | 0.370 | 1.53× |
| 1,000 | 10 | 0.505 | 1.113 | 2.20× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
