# AccumulationDistributionOscillator benchmark (`ADOSC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 68.98M | 0.013 | 79.34M | 0.038 | 2.62× | 3.01× |
| 10,000 | 0.082 | 121.60M | 0.076 | 132.28M | 0.059 | 0.71× | 0.77× |
| 100,000 | 0.747 | 133.95M | 0.740 | 135.12M | 0.278 | 0.37× | 0.38× |
| 1,000,000 | 7.992 | 125.13M | 7.534 | 132.73M | 3.025 | 0.38× | 0.40× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.161 | 0.174 | 1.08× |
| 1 | 5 | 0.337 | 0.502 | 1.49× |
| 1 | 10 | 0.490 | 0.945 | 1.93× |
| 10 | 1 | 0.054 | 0.094 | 1.74× |
| 10 | 5 | 0.240 | 0.454 | 1.90× |
| 10 | 10 | 0.524 | 0.943 | 1.80× |
| 100 | 1 | 0.052 | 0.106 | 2.03× |
| 100 | 5 | 0.234 | 0.486 | 2.08× |
| 100 | 10 | 0.512 | 1.007 | 1.97× |
| 1,000 | 1 | 0.059 | 0.102 | 1.72× |
| 1,000 | 5 | 0.259 | 0.480 | 1.85× |
| 1,000 | 10 | 0.541 | 0.995 | 1.84× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
