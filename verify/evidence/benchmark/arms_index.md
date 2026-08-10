# ArmsIndex benchmark (`Trin` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 85.91M | 0.009 | 111.93M | 8.020 | 688.99× | 897.71× |
| 10,000 | 0.042 | 236.14M | 0.037 | 266.86M | 84.144 | 1986.97× | 2245.44× |
| 100,000 | 0.365 | 274.20M | 0.329 | 304.17M | 816.063 | 2237.64× | 2482.20× |
| 1,000,000 | 4.212 | 237.39M | 3.649 | 274.08M | 8201.289 | 1946.94× | 2247.84× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.191 | 0.360 | 1.89× |
| 1 | 5 | 0.322 | 1.121 | 3.48× |
| 1 | 10 | 0.521 | 2.402 | 4.61× |
| 10 | 1 | 0.056 | 0.306 | 5.46× |
| 10 | 5 | 0.240 | 1.504 | 6.27× |
| 10 | 10 | 0.529 | 3.255 | 6.16× |
| 100 | 1 | 0.053 | 1.088 | 20.43× |
| 100 | 5 | 0.256 | 5.512 | 21.52× |
| 100 | 10 | 0.530 | 11.302 | 21.33× |
| 1,000 | 1 | 0.057 | 8.595 | 150.50× |
| 1,000 | 5 | 0.351 | 44.034 | 125.56× |
| 1,000 | 10 | 0.699 | 93.243 | 133.31× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
