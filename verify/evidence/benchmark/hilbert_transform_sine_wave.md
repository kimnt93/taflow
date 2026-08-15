# HilbertTransformSineWave benchmark (`HT_SINE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.138 | 7.26M | 0.130 | 7.68M | 0.494 | 3.59× | 3.79× |
| 10,000 | 1.336 | 7.49M | 1.365 | 7.33M | 4.606 | 3.45× | 3.37× |
| 100,000 | 13.717 | 7.29M | 13.796 | 7.25M | 50.226 | 3.66× | 3.64× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.069 | 0.120 | 1.73× |
| 1 | 5 | 0.217 | 0.496 | 2.29× |
| 1 | 10 | 0.424 | 0.949 | 2.24× |
| 10 | 1 | 0.040 | 0.104 | 2.58× |
| 10 | 5 | 0.187 | 0.433 | 2.32× |
| 10 | 10 | 0.433 | 0.984 | 2.27× |
| 100 | 1 | 0.059 | 0.119 | 2.02× |
| 100 | 5 | 0.191 | 0.588 | 3.08× |
| 100 | 10 | 0.415 | 1.219 | 2.94× |
| 1,000 | 1 | 0.187 | 0.560 | 3.00× |
| 1,000 | 5 | 0.297 | 2.804 | 9.44× |
| 1,000 | 10 | 0.596 | 5.641 | 9.46× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
