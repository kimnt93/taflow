# HilbertTransformSineWave benchmark (`HT_SINE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.136 | 7.36M | 0.129 | 7.76M | 0.461 | 3.39× | 3.58× |
| 10,000 | 1.434 | 6.97M | 1.356 | 7.37M | 4.315 | 3.01× | 3.18× |
| 100,000 | 14.011 | 7.14M | 13.903 | 7.19M | 43.511 | 3.11× | 3.13× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.085 | 0.105 | 1.25× |
| 1 | 5 | 0.342 | 0.476 | 1.39× |
| 1 | 10 | 0.457 | 0.931 | 2.04× |
| 10 | 1 | 0.051 | 0.088 | 1.73× |
| 10 | 5 | 0.222 | 0.443 | 1.99× |
| 10 | 10 | 0.471 | 0.917 | 1.95× |
| 100 | 1 | 0.056 | 0.123 | 2.20× |
| 100 | 5 | 0.242 | 0.569 | 2.35× |
| 100 | 10 | 0.483 | 1.183 | 2.45× |
| 1,000 | 1 | 0.188 | 0.545 | 2.89× |
| 1,000 | 5 | 0.326 | 2.697 | 8.27× |
| 1,000 | 10 | 0.571 | 5.636 | 9.88× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
