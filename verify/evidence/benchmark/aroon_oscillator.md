# AroonOscillator benchmark (`AROONOSC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 109.18M | 0.008 | 122.88M | 0.036 | 3.89× | 4.38× |
| 10,000 | 0.103 | 96.95M | 0.103 | 97.47M | 0.125 | 1.22× | 1.22× |
| 100,000 | 1.094 | 91.42M | 0.997 | 100.27M | 1.006 | 0.92× | 1.01× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.080 | 0.110 | 1.37× |
| 1 | 5 | 0.274 | 0.504 | 1.84× |
| 1 | 10 | 0.385 | 0.889 | 2.31× |
| 10 | 1 | 0.041 | 0.095 | 2.31× |
| 10 | 5 | 0.172 | 0.410 | 2.38× |
| 10 | 10 | 0.382 | 0.883 | 2.31× |
| 100 | 1 | 0.040 | 0.093 | 2.31× |
| 100 | 5 | 0.194 | 0.423 | 2.19× |
| 100 | 10 | 0.380 | 0.903 | 2.37× |
| 1,000 | 1 | 0.055 | 0.096 | 1.73× |
| 1,000 | 5 | 0.186 | 0.467 | 2.51× |
| 1,000 | 10 | 0.399 | 1.022 | 2.56× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
