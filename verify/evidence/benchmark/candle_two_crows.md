# CandleTwoCrows benchmark (`CDL2CROWS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 95.88M | 0.007 | 133.83M | 0.029 | 2.82× | 3.93× |
| 10,000 | 0.066 | 151.18M | 0.062 | 160.87M | 0.104 | 1.57× | 1.68× |
| 100,000 | 0.890 | 112.37M | 0.892 | 112.11M | 0.916 | 1.03× | 1.03× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.085 | 0.119 | 1.41× |
| 1 | 5 | 0.281 | 0.434 | 1.54× |
| 1 | 10 | 0.520 | 0.924 | 1.78× |
| 10 | 1 | 0.053 | 0.091 | 1.73× |
| 10 | 5 | 0.253 | 0.421 | 1.66× |
| 10 | 10 | 0.508 | 0.891 | 1.75× |
| 100 | 1 | 0.055 | 0.095 | 1.73× |
| 100 | 5 | 0.271 | 0.440 | 1.62× |
| 100 | 10 | 0.548 | 0.902 | 1.65× |
| 1,000 | 1 | 0.065 | 0.104 | 1.59× |
| 1,000 | 5 | 0.263 | 0.473 | 1.80× |
| 1,000 | 10 | 0.560 | 1.001 | 1.79× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
