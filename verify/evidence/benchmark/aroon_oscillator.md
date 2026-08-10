# AroonOscillator benchmark (`AROONOSC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.033 | 30.39M | 0.032 | 30.85M | 0.042 | 1.28× | 1.30× |
| 10,000 | 0.309 | 32.35M | 0.329 | 30.37M | 0.158 | 0.51× | 0.48× |
| 100,000 | 3.527 | 28.35M | 4.770 | 20.97M | 1.578 | 0.45× | 0.33× |
| 1,000,000 | 35.039 | 28.54M | 33.116 | 30.20M | 11.078 | 0.32× | 0.33× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.072 | 0.118 | 1.63× |
| 1 | 5 | 0.333 | 0.528 | 1.58× |
| 1 | 10 | 0.535 | 1.070 | 2.00× |
| 10 | 1 | 0.061 | 0.107 | 1.75× |
| 10 | 5 | 0.318 | 0.549 | 1.72× |
| 10 | 10 | 0.542 | 1.210 | 2.23× |
| 100 | 1 | 0.086 | 0.157 | 1.83× |
| 100 | 5 | 0.312 | 0.568 | 1.82× |
| 100 | 10 | 0.591 | 1.046 | 1.77× |
| 1,000 | 1 | 0.085 | 0.107 | 1.27× |
| 1,000 | 5 | 0.321 | 0.630 | 1.96× |
| 1,000 | 10 | 0.640 | 1.254 | 1.96× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
