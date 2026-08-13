# CandleKicking benchmark (`CDLKICKING` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.136 | 7.36M | 0.133 | 7.51M | 0.038 | 0.28× | 0.29× |
| 10,000 | 1.185 | 8.44M | 1.141 | 8.76M | 0.164 | 0.14× | 0.14× |
| 100,000 | 11.456 | 8.73M | 11.525 | 8.68M | 1.413 | 0.12× | 0.12× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.144 | 0.104 | 0.73× |
| 1 | 5 | 0.345 | 0.479 | 1.39× |
| 1 | 10 | 0.640 | 0.924 | 1.44× |
| 10 | 1 | 0.067 | 0.093 | 1.40× |
| 10 | 5 | 0.315 | 0.434 | 1.38× |
| 10 | 10 | 0.653 | 0.902 | 1.38× |
| 100 | 1 | 0.082 | 0.088 | 1.07× |
| 100 | 5 | 0.312 | 0.428 | 1.37× |
| 100 | 10 | 0.670 | 0.929 | 1.39× |
| 1,000 | 1 | 0.195 | 0.104 | 0.53× |
| 1,000 | 5 | 0.354 | 0.502 | 1.42× |
| 1,000 | 10 | 0.742 | 1.066 | 1.44× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
