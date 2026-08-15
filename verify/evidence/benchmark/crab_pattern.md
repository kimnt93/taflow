# CrabPattern benchmark (`Crab` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 94.84M | 0.008 | 130.45M | 0.239 | 22.68× | 31.20× |
| 10,000 | 0.097 | 103.30M | 0.098 | 102.22M | 1.361 | 14.06× | 13.91× |
| 100,000 | 0.890 | 112.31M | 0.873 | 114.53M | 13.027 | 14.63× | 14.92× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.083 | 0.228 | 2.75× |
| 1 | 5 | 0.238 | 0.823 | 3.46× |
| 1 | 10 | 0.384 | 1.641 | 4.27× |
| 10 | 1 | 0.047 | 0.167 | 3.53× |
| 10 | 5 | 0.226 | 1.102 | 4.88× |
| 10 | 10 | 0.410 | 1.657 | 4.05× |
| 100 | 1 | 0.049 | 0.192 | 3.95× |
| 100 | 5 | 0.245 | 1.173 | 4.78× |
| 100 | 10 | 0.420 | 1.788 | 4.25× |
| 1,000 | 1 | 0.076 | 0.309 | 4.09× |
| 1,000 | 5 | 0.213 | 1.774 | 8.33× |
| 1,000 | 10 | 0.435 | 3.051 | 7.02× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
