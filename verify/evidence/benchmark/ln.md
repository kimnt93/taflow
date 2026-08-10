# MathLn benchmark (`LN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 128.23M | 0.006 | 162.42M | 0.030 | 3.89× | 4.92× |
| 10,000 | 0.047 | 213.78M | 0.044 | 225.28M | 0.066 | 1.42× | 1.50× |
| 100,000 | 0.448 | 223.18M | 0.424 | 235.96M | 0.427 | 0.95× | 1.01× |
| 1,000,000 | 4.759 | 210.11M | 4.177 | 239.43M | 4.275 | 0.90× | 1.02× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.141 | 0.136 | 0.97× |
| 1 | 5 | 0.240 | 0.442 | 1.84× |
| 1 | 10 | 0.491 | 0.918 | 1.87× |
| 10 | 1 | 0.046 | 0.081 | 1.77× |
| 10 | 5 | 0.233 | 0.417 | 1.80× |
| 10 | 10 | 0.541 | 0.898 | 1.66× |
| 100 | 1 | 0.051 | 0.086 | 1.67× |
| 100 | 5 | 0.250 | 0.434 | 1.74× |
| 100 | 10 | 0.487 | 0.917 | 1.88× |
| 1,000 | 1 | 0.049 | 0.087 | 1.78× |
| 1,000 | 5 | 0.232 | 0.454 | 1.95× |
| 1,000 | 10 | 0.501 | 0.939 | 1.88× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
