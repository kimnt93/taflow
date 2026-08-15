# ExponentiallyWeightedCorrelation benchmark (`ewm correlation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 134.33M | 0.006 | 161.67M | 1.252 | 168.23× | 202.46× |
| 10,000 | 0.056 | 179.57M | 0.053 | 187.49M | 12.503 | 224.51× | 234.42× |
| 100,000 | 0.514 | 194.66M | 0.496 | 201.74M | 125.388 | 244.08× | 252.96× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.080 | 0.121 | 1.52× |
| 1 | 5 | 0.259 | 0.539 | 2.08× |
| 1 | 10 | 0.457 | 0.985 | 2.16× |
| 10 | 1 | 0.046 | 0.108 | 2.32× |
| 10 | 5 | 0.190 | 0.548 | 2.88× |
| 10 | 10 | 0.428 | 1.142 | 2.67× |
| 100 | 1 | 0.042 | 0.226 | 5.41× |
| 100 | 5 | 0.209 | 1.152 | 5.50× |
| 100 | 10 | 0.412 | 2.254 | 5.47× |
| 1,000 | 1 | 0.053 | 1.361 | 25.79× |
| 1,000 | 5 | 0.197 | 7.042 | 35.75× |
| 1,000 | 10 | 0.414 | 14.518 | 35.04× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
