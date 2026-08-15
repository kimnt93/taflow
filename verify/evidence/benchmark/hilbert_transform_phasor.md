# HilbertTransformPhasor benchmark (`HT_PHASOR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.047 | 21.49M | 0.046 | 21.77M | 0.079 | 1.69× | 1.71× |
| 10,000 | 0.451 | 22.16M | 0.437 | 22.89M | 0.464 | 1.03× | 1.06× |
| 100,000 | 4.647 | 21.52M | 4.537 | 22.04M | 4.437 | 0.95× | 0.98× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.086 | 0.114 | 1.32× |
| 1 | 5 | 0.310 | 0.500 | 1.61× |
| 1 | 10 | 0.437 | 0.974 | 2.23× |
| 10 | 1 | 0.042 | 0.090 | 2.13× |
| 10 | 5 | 0.189 | 0.457 | 2.42× |
| 10 | 10 | 0.387 | 1.020 | 2.63× |
| 100 | 1 | 0.051 | 0.103 | 2.02× |
| 100 | 5 | 0.203 | 0.479 | 2.36× |
| 100 | 10 | 0.430 | 1.027 | 2.39× |
| 1,000 | 1 | 0.098 | 0.164 | 1.68× |
| 1,000 | 5 | 0.243 | 0.737 | 3.04× |
| 1,000 | 10 | 0.471 | 1.406 | 2.99× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
