# HilbertTransformDominantCyclePhase benchmark (`HT_DCPHASE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.102 | 9.85M | 0.101 | 9.90M | 0.448 | 4.41× | 4.43× |
| 10,000 | 1.092 | 9.15M | 1.044 | 9.58M | 4.504 | 4.12× | 4.32× |
| 100,000 | 10.656 | 9.38M | 10.629 | 9.41M | 45.589 | 4.28× | 4.29× |
| 1,000,000 | 116.014 | 8.62M | 109.107 | 9.17M | 414.701 | 3.57× | 3.80× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.113 | 0.116 | 1.03× |
| 1 | 5 | 0.287 | 0.464 | 1.62× |
| 1 | 10 | 0.470 | 0.946 | 2.01× |
| 10 | 1 | 0.049 | 0.090 | 1.81× |
| 10 | 5 | 0.217 | 0.406 | 1.87× |
| 10 | 10 | 0.458 | 0.903 | 1.97× |
| 100 | 1 | 0.053 | 0.112 | 2.10× |
| 100 | 5 | 0.230 | 0.565 | 2.46× |
| 100 | 10 | 0.482 | 1.161 | 2.41× |
| 1,000 | 1 | 0.155 | 0.515 | 3.31× |
| 1,000 | 5 | 0.296 | 2.614 | 8.85× |
| 1,000 | 10 | 0.569 | 5.271 | 9.27× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
