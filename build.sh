#!/bin/bash

CMD="rustc -O -Cprefer-dynamic test_qt.rs -o quadtree"

echo "$CMD"
$CMD
