# RollingAlpha benchmark (`Alpha` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.286 | 3.50M | 0.283 | 3.53M | 0.229 | 0.80× | 0.81× |
| 10,000 | 2.774 | 3.60M | 2.799 | 3.57M | 0.911 | 0.33× | 0.33× |
| 100,000 | 27.952 | 3.58M | 27.929 | 3.58M | 7.846 | 0.28× | 0.28× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.091 | 0.259 | 2.84× |
| 1 | 5 | 0.376 | 1.192 | 3.17× |
| 1 | 10 | 0.662 | 2.463 | 3.72× |
| 10 | 1 | 0.072 | 0.235 | 3.25× |
| 10 | 5 | 0.316 | 1.396 | 4.42× |
| 10 | 10 | 0.618 | 2.461 | 3.98× |
| 100 | 1 | 0.097 | 0.240 | 2.48× |
| 100 | 5 | 0.312 | 1.447 | 4.64× |
| 100 | 10 | 0.669 | 2.555 | 3.82× |
| 1,000 | 1 | 0.359 | 0.307 | 0.86× |
| 1,000 | 5 | 0.526 | 1.804 | 3.43× |
| 1,000 | 10 | 0.968 | 3.407 | 3.52× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
