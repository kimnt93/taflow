# MathCos benchmark (`COS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 70.34M | 0.013 | 77.63M | 0.044 | 3.08× | 3.39× |
| 10,000 | 0.161 | 62.16M | 0.152 | 65.85M | 0.189 | 1.18× | 1.25× |
| 100,000 | 1.818 | 55.01M | 1.573 | 63.56M | 1.620 | 0.89× | 1.03× |
| 1,000,000 | 17.451 | 57.30M | 16.522 | 60.52M | 17.208 | 0.99× | 1.04× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.092 | 0.127 | 1.39× |
| 1 | 5 | 0.302 | 0.528 | 1.75× |
| 1 | 10 | 0.584 | 1.109 | 1.90× |
| 10 | 1 | 0.055 | 0.098 | 1.77× |
| 10 | 5 | 0.317 | 0.496 | 1.56× |
| 10 | 10 | 0.529 | 1.070 | 2.02× |
| 100 | 1 | 0.061 | 0.091 | 1.50× |
| 100 | 5 | 0.270 | 0.448 | 1.66× |
| 100 | 10 | 0.454 | 0.967 | 2.13× |
| 1,000 | 1 | 0.076 | 0.115 | 1.52× |
| 1,000 | 5 | 0.354 | 0.622 | 1.76× |
| 1,000 | 10 | 0.515 | 1.089 | 2.12× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
