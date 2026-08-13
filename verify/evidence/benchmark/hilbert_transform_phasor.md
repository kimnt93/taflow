# HilbertTransformPhasor benchmark (`HT_PHASOR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.219 | 4.57M | 0.179 | 5.58M | 0.072 | 0.33× | 0.40× |
| 10,000 | 1.245 | 8.03M | 1.235 | 8.10M | 0.454 | 0.36× | 0.37× |
| 100,000 | 12.994 | 7.70M | 12.443 | 8.04M | 4.195 | 0.32× | 0.34× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.097 | 0.146 | 1.51× |
| 1 | 5 | 0.403 | 0.512 | 1.27× |
| 1 | 10 | 0.617 | 0.925 | 1.50× |
| 10 | 1 | 0.079 | 0.095 | 1.19× |
| 10 | 5 | 0.323 | 0.465 | 1.44× |
| 10 | 10 | 0.639 | 0.933 | 1.46× |
| 100 | 1 | 0.080 | 0.099 | 1.23× |
| 100 | 5 | 0.298 | 0.461 | 1.55× |
| 100 | 10 | 0.662 | 0.972 | 1.47× |
| 1,000 | 1 | 0.198 | 0.145 | 0.73× |
| 1,000 | 5 | 0.348 | 0.669 | 1.92× |
| 1,000 | 10 | 0.699 | 1.410 | 2.02× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
