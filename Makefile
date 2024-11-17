ORIGINAL_PATH_TEST_1 =/home/roothunter/Dev/tc-cli/tests/inputs/input_1.txt
COMPRESSED_PATH_TEST_1 =/home/roothunter/Dev/tc-cli/tests/tmp/test_1_compressed.tc
DECOMPRESSED_PATH_TEST_1 =/home/roothunter/Dev/tc-cli/tests/tmp/test_1_decompressed.tc

tc-update:
	cargo update -p tc

test-1-compress:
	cargo run -- -c -i ${ORIGINAL_PATH_TEST_1} -o ${COMPRESSED_PATH_TEST_1}

test-1-decompress:
	cargo run -- -d -i ${COMPRESSED_PATH_TEST_1} -o ${DECOMPRESSED_PATH_TEST_1}

test-1-all: tc-update
	cargo run -- -c -i ${ORIGINAL_PATH_TEST_1} -o ${COMPRESSED_PATH_TEST_1}
	cargo run -- -d -i ${COMPRESSED_PATH_TEST_1} -o ${DECOMPRESSED_PATH_TEST_1}

	@echo "Check files diffs:";
	
	@if diff -q ${ORIGINAL_PATH_TEST_1} ${DECOMPRESSED_PATH_TEST_1} > /dev/null; then \
        echo "The files are equal"; \
    else \
        echo "The files are different"; \
    fi