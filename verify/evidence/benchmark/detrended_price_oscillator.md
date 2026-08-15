# DetrendedPriceOscillator benchmark (`dpo` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 153.73M | 0.006 | 177.91M | 0.328 | 50.46× | 58.39× |
| 10,000 | 0.051 | 196.34M | 0.045 | 223.52M | 0.408 | 8.00× | 9.11× |
| 100,000 | 0.461 | 216.71M | 0.423 | 236.57M | 1.349 | 2.92× | 3.19× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.080 | 0.165 | 2.06× |
| 1 | 5 | 0.286 | 0.752 | 2.63× |
| 1 | 10 | 0.419 | 1.582 | 3.78× |
| 10 | 1 | 0.047 | 0.155 | 3.32× |
| 10 | 5 | 0.180 | 0.726 | 4.03× |
| 10 | 10 | 0.395 | 1.602 | 4.05× |
| 100 | 1 | 0.048 | 0.431 | 8.98× |
| 100 | 5 | 0.205 | 1.938 | 9.46× |
| 100 | 10 | 0.447 | 4.001 | 8.96× |
| 1,000 | 1 | 0.058 | 0.402 | 6.93× |
| 1,000 | 5 | 0.218 | 2.087 | 9.60× |
| 1,000 | 10 | 0.414 | 4.166 | 10.06× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
