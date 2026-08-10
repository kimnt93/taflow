# HilbertTransformPhasor benchmark (`HT_PHASOR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.054 | 18.36M | 0.057 | 17.50M | 0.090 | 1.65× | 1.57× |
| 10,000 | 0.531 | 18.82M | 0.518 | 19.29M | 0.502 | 0.94× | 0.97× |
| 100,000 | 5.052 | 19.79M | 5.258 | 19.02M | 4.877 | 0.97× | 0.93× |
| 1,000,000 | 51.840 | 19.29M | 51.686 | 19.35M | 47.063 | 0.91× | 0.91× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.065 | 0.116 | 1.77× |
| 1 | 5 | 0.270 | 0.581 | 2.15× |
| 1 | 10 | 0.591 | 1.037 | 1.76× |
| 10 | 1 | 0.059 | 0.114 | 1.91× |
| 10 | 5 | 0.274 | 0.575 | 2.10× |
| 10 | 10 | 0.547 | 0.995 | 1.82× |
| 100 | 1 | 0.053 | 0.104 | 1.97× |
| 100 | 5 | 0.249 | 0.534 | 2.15× |
| 100 | 10 | 0.573 | 1.114 | 1.94× |
| 1,000 | 1 | 0.103 | 0.139 | 1.34× |
| 1,000 | 5 | 0.243 | 0.834 | 3.43× |
| 1,000 | 10 | 0.613 | 1.525 | 2.49× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
