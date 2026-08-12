# CandleMatchingLow benchmark (`CDLMATCHINGLOW` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 57.70M | 0.015 | 67.50M | 0.032 | 1.84× | 2.15× |
| 10,000 | 0.104 | 96.07M | 0.102 | 97.73M | 0.091 | 0.87× | 0.89× |
| 100,000 | 0.965 | 103.58M | 1.020 | 98.08M | 0.669 | 0.69× | 0.66× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.141 | 0.149 | 1.06× |
| 1 | 5 | 0.324 | 0.463 | 1.43× |
| 1 | 10 | 0.543 | 0.951 | 1.75× |
| 10 | 1 | 0.059 | 0.097 | 1.64× |
| 10 | 5 | 0.245 | 0.486 | 1.98× |
| 10 | 10 | 0.568 | 0.897 | 1.58× |
| 100 | 1 | 0.059 | 0.087 | 1.48× |
| 100 | 5 | 0.277 | 0.440 | 1.59× |
| 100 | 10 | 0.565 | 1.040 | 1.84× |
| 1,000 | 1 | 0.065 | 0.099 | 1.53× |
| 1,000 | 5 | 0.284 | 0.511 | 1.80× |
| 1,000 | 10 | 0.578 | 1.101 | 1.90× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
