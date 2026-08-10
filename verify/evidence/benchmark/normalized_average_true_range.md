# NormalizedAverageTrueRange benchmark (`NATR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 79.82M | 0.011 | 88.91M | 0.037 | 2.96× | 3.29× |
| 10,000 | 0.077 | 129.90M | 0.069 | 144.50M | 0.088 | 1.14× | 1.27× |
| 100,000 | 0.666 | 150.16M | 0.642 | 155.68M | 0.577 | 0.87× | 0.90× |
| 1,000,000 | 7.455 | 134.14M | 8.198 | 121.98M | 6.419 | 0.86× | 0.78× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.080 | 0.125 | 1.56× |
| 1 | 5 | 0.234 | 0.462 | 1.97× |
| 1 | 10 | 0.481 | 0.939 | 1.95× |
| 10 | 1 | 0.050 | 0.095 | 1.89× |
| 10 | 5 | 0.262 | 0.450 | 1.72× |
| 10 | 10 | 0.492 | 0.923 | 1.88× |
| 100 | 1 | 0.051 | 0.098 | 1.92× |
| 100 | 5 | 0.237 | 0.447 | 1.89× |
| 100 | 10 | 0.498 | 0.924 | 1.86× |
| 1,000 | 1 | 0.060 | 0.104 | 1.74× |
| 1,000 | 5 | 0.246 | 0.483 | 1.97× |
| 1,000 | 10 | 0.503 | 0.990 | 1.97× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
