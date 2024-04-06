docker run --rm -v %cd%:/app ^
    -v %cd%/test_data:/pictures ^
    -e CARGO_PROFILE_RELEASE_DEBUG=false ^
    png_reader /bin/bash -c ^
    "cargo flamegraph --bench bench --release --flamechart -- bench_load_data_big_white bench_load_data_big_white_png 2> errlog.txt 1> log.txt"

