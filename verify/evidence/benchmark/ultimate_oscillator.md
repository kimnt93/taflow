# UltimateOscillator benchmark (`ULTOSC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 48.83M | 0.017 | 59.02M | 0.051 | 2.50× | 3.02× |
| 10,000 | 0.137 | 73.08M | 0.142 | 70.51M | 0.182 | 1.33× | 1.28× |
| 100,000 | 1.356 | 73.72M | 1.278 | 78.23M | 1.522 | 1.12× | 1.19× |
| 1,000,000 | 14.770 | 67.70M | 13.178 | 75.88M | 15.426 | 1.04× | 1.17× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.129 | 0.112 | 0.86× |
| 1 | 5 | 0.388 | 0.511 | 1.32× |
| 1 | 10 | 0.511 | 0.947 | 1.85× |
| 10 | 1 | 0.053 | 0.098 | 1.86× |
| 10 | 5 | 0.226 | 0.473 | 2.10× |
| 10 | 10 | 0.503 | 0.998 | 1.98× |
| 100 | 1 | 0.055 | 0.096 | 1.76× |
| 100 | 5 | 0.246 | 0.481 | 1.96× |
| 100 | 10 | 0.552 | 1.043 | 1.89× |
| 1,000 | 1 | 0.067 | 0.111 | 1.66× |
| 1,000 | 5 | 0.250 | 0.569 | 2.27× |
| 1,000 | 10 | 0.554 | 1.259 | 2.27× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
