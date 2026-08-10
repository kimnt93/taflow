# CandleHaramiCross benchmark (`CDLHARAMICROSS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.022 | 44.49M | 0.019 | 53.33M | 0.037 | 1.66× | 1.99× |
| 10,000 | 0.161 | 61.94M | 0.156 | 64.22M | 0.147 | 0.91× | 0.95× |
| 100,000 | 1.576 | 63.43M | 1.548 | 64.62M | 1.290 | 0.82× | 0.83× |
| 1,000,000 | 17.016 | 58.77M | 16.271 | 61.46M | 11.601 | 0.68× | 0.71× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.121 | 0.126 | 1.04× |
| 1 | 5 | 0.272 | 0.486 | 1.79× |
| 1 | 10 | 0.527 | 0.890 | 1.69× |
| 10 | 1 | 0.054 | 0.087 | 1.63× |
| 10 | 5 | 0.241 | 0.421 | 1.75× |
| 10 | 10 | 0.588 | 0.911 | 1.55× |
| 100 | 1 | 0.057 | 0.087 | 1.52× |
| 100 | 5 | 0.253 | 0.421 | 1.66× |
| 100 | 10 | 0.526 | 1.013 | 1.92× |
| 1,000 | 1 | 0.073 | 0.098 | 1.34× |
| 1,000 | 5 | 0.293 | 0.521 | 1.78× |
| 1,000 | 10 | 0.563 | 1.058 | 1.88× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
