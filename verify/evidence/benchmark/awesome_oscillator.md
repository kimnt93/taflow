# AwesomeOscillator benchmark (`AwesomeOscillator` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.025 | 40.54M | 0.023 | 42.96M | 0.216 | 8.74× | 9.26× |
| 10,000 | 0.228 | 43.85M | 0.226 | 44.16M | 0.870 | 3.81× | 3.84× |
| 100,000 | 2.274 | 43.98M | 2.230 | 44.85M | 6.689 | 2.94× | 3.00× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.091 | 0.283 | 3.10× |
| 1 | 5 | 0.265 | 1.323 | 4.99× |
| 1 | 10 | 0.394 | 2.868 | 7.27× |
| 10 | 1 | 0.051 | 0.326 | 6.36× |
| 10 | 5 | 0.217 | 1.582 | 7.30× |
| 10 | 10 | 0.416 | 2.517 | 6.04× |
| 100 | 1 | 0.049 | 0.250 | 5.11× |
| 100 | 5 | 0.206 | 1.406 | 6.82× |
| 100 | 10 | 0.426 | 2.742 | 6.44× |
| 1,000 | 1 | 0.072 | 0.313 | 4.37× |
| 1,000 | 5 | 0.202 | 1.733 | 8.56× |
| 1,000 | 10 | 0.443 | 3.174 | 7.16× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
