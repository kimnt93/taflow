# HilbertTransformPhasor benchmark (`HT_PHASOR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.053 | 19.02M | 0.052 | 19.07M | 0.080 | 1.53× | 1.53× |
| 10,000 | 0.489 | 20.46M | 0.477 | 20.96M | 0.486 | 0.99× | 1.02× |
| 100,000 | 4.869 | 20.54M | 4.734 | 21.12M | 4.781 | 0.98× | 1.01× |
| 1,000,000 | 48.888 | 20.45M | 48.618 | 20.57M | 46.208 | 0.95× | 0.95× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.061 | 0.132 | 2.16× |
| 1 | 5 | 0.259 | 0.496 | 1.92× |
| 1 | 10 | 0.502 | 1.038 | 2.07× |
| 10 | 1 | 0.048 | 0.089 | 1.84× |
| 10 | 5 | 0.229 | 0.461 | 2.02× |
| 10 | 10 | 0.451 | 1.006 | 2.23× |
| 100 | 1 | 0.062 | 0.095 | 1.53× |
| 100 | 5 | 0.237 | 0.474 | 2.00× |
| 100 | 10 | 0.488 | 0.940 | 1.93× |
| 1,000 | 1 | 0.094 | 0.134 | 1.42× |
| 1,000 | 5 | 0.259 | 0.723 | 2.78× |
| 1,000 | 10 | 0.496 | 1.398 | 2.82× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
