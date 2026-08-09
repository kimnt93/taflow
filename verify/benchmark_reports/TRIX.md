# TripleExponentialRateOfChange benchmark (`TRIX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 155.68M | 0.005 | 182.44M | 0.040 | 6.27× | 7.34× |
| 10,000 | 0.029 | 348.55M | 0.026 | 385.63M | 0.129 | 4.50× | 4.98× |
| 100,000 | 0.268 | 373.18M | 0.234 | 426.94M | 0.952 | 3.55× | 4.06× |
| 1,000,000 | 3.003 | 332.98M | 2.464 | 405.85M | 10.043 | 3.34× | 4.08× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.085 | 0.129 | 1.51× |
| 1 | 5 | 0.330 | 0.520 | 1.57× |
| 1 | 10 | 0.528 | 1.017 | 1.93× |
| 10 | 1 | 0.047 | 0.089 | 1.88× |
| 10 | 5 | 0.240 | 0.460 | 1.91× |
| 10 | 10 | 0.547 | 1.023 | 1.87× |
| 100 | 1 | 0.061 | 0.095 | 1.56× |
| 100 | 5 | 0.230 | 0.452 | 1.96× |
| 100 | 10 | 0.501 | 1.070 | 2.13× |
| 1,000 | 1 | 0.066 | 0.105 | 1.59× |
| 1,000 | 5 | 0.251 | 0.544 | 2.17× |
| 1,000 | 10 | 0.500 | 1.038 | 2.07× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
