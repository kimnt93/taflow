# MathCbrt benchmark (`numpy.cbrt` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 57.10M | 0.017 | 59.26M | 0.026 | 1.50× | 1.56× |
| 10,000 | 0.162 | 61.76M | 0.158 | 63.27M | 0.148 | 0.91× | 0.94× |
| 100,000 | 1.584 | 63.14M | 1.578 | 63.37M | 1.332 | 0.84× | 0.84× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.155 | 0.093 | 0.60× |
| 1 | 5 | 0.337 | 0.268 | 0.80× |
| 1 | 10 | 0.360 | 0.567 | 1.58× |
| 10 | 1 | 0.042 | 0.058 | 1.39× |
| 10 | 5 | 0.182 | 0.268 | 1.47× |
| 10 | 10 | 0.369 | 0.554 | 1.50× |
| 100 | 1 | 0.041 | 0.060 | 1.46× |
| 100 | 5 | 0.179 | 0.270 | 1.50× |
| 100 | 10 | 0.380 | 0.555 | 1.46× |
| 1,000 | 1 | 0.057 | 0.071 | 1.25× |
| 1,000 | 5 | 0.189 | 0.307 | 1.62× |
| 1,000 | 10 | 0.437 | 0.697 | 1.59× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
