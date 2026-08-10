# AroonOscillator benchmark (`AROONOSC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.032 | 31.02M | 0.030 | 32.93M | 0.040 | 1.24× | 1.32× |
| 10,000 | 0.293 | 34.09M | 0.297 | 33.66M | 0.148 | 0.50× | 0.50× |
| 100,000 | 3.533 | 28.30M | 3.231 | 30.95M | 1.329 | 0.38× | 0.41× |
| 1,000,000 | 31.547 | 31.70M | 30.716 | 32.56M | 11.141 | 0.35× | 0.36× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.082 | 0.146 | 1.79× |
| 1 | 5 | 0.292 | 0.546 | 1.87× |
| 1 | 10 | 0.519 | 0.991 | 1.91× |
| 10 | 1 | 0.052 | 0.096 | 1.84× |
| 10 | 5 | 0.234 | 0.502 | 2.15× |
| 10 | 10 | 0.507 | 1.016 | 2.00× |
| 100 | 1 | 0.056 | 0.098 | 1.76× |
| 100 | 5 | 0.243 | 0.504 | 2.07× |
| 100 | 10 | 0.574 | 1.044 | 1.82× |
| 1,000 | 1 | 0.082 | 0.108 | 1.32× |
| 1,000 | 5 | 0.274 | 0.551 | 2.01× |
| 1,000 | 10 | 0.578 | 1.109 | 1.92× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
