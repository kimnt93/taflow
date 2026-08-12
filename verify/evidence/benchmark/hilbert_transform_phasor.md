# HilbertTransformPhasor benchmark (`HT_PHASOR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.047 | 21.41M | 0.045 | 22.20M | 0.071 | 1.52× | 1.57× |
| 10,000 | 0.425 | 23.54M | 0.425 | 23.52M | 0.438 | 1.03× | 1.03× |
| 100,000 | 4.484 | 22.30M | 4.302 | 23.24M | 4.105 | 0.92× | 0.95× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.088 | 0.098 | 1.12× |
| 1 | 5 | 0.328 | 0.454 | 1.38× |
| 1 | 10 | 0.456 | 0.937 | 2.06× |
| 10 | 1 | 0.050 | 0.091 | 1.81× |
| 10 | 5 | 0.210 | 0.446 | 2.12× |
| 10 | 10 | 0.463 | 0.930 | 2.01× |
| 100 | 1 | 0.052 | 0.096 | 1.86× |
| 100 | 5 | 0.237 | 0.450 | 1.90× |
| 100 | 10 | 0.465 | 0.976 | 2.10× |
| 1,000 | 1 | 0.098 | 0.138 | 1.40× |
| 1,000 | 5 | 0.239 | 0.708 | 2.96× |
| 1,000 | 10 | 0.618 | 1.528 | 2.47× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
