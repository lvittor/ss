
INPUT_FILE_PATH := TP3/data/input.txt
OUTPUT_FILE_PATH := TP3/data/output.txt
CAPTURE_DIR := 
RUN_ARGS :=
MAX_TIME :=
PRECISION := F64
FEATURES :=

ifeq "$(PRECISION)" "F64"
	FEATURES := "use_f64"
else
	FEATURES := "use_f32"
endif


build:
	make -C .. -f Makefile.rust build PACKAGE=tp3 "FEATURE_ARGS=--no-default-features --features=$(FEATURES)"

run-with-vis: build
	make -C .. -f Makefile.rust -s run-raw BIN=simulation \
		ARGS="--input $(INPUT_FILE_PATH) $(if $(MAX_TIME),--max-duration $(MAX_TIME))" | \
	make -C .. -f Makefile.rust run-raw BIN=visualization \
		ARGS="--input $(INPUT_FILE_PATH) --output /dev/stdin $(if $(CAPTURE_DIR),--capture-directory $(CAPTURE_DIR))"

run-raw:
	make -C .. -f Makefile.rust -s run-raw PACKAGE=tp3 ARGS="$(RUN_ARGS)"

generate-precision-diff-video:
	make run-with-vis USE_DOCKER=FALSE PRECISION=F32 CAPTURE_DIR=TP3/f32_capture
	make run-with-vis USE_DOCKER=FALSE PRECISION=F64 CAPTURE_DIR=TP3/f64_capture
	ffmpeg \
		-framerate 60 -pattern_type glob -i 'f64_capture/*.png' \
		-framerate 60 -pattern_type glob -i 'f32_capture/*.png' \
		-filter_complex "[1:v]chromakey=0x305a4a:0.01:0.1[ckout];[0:v][ckout]blend=all_mode='overlay':all_opacity=0.7 [out]" -map "[out]" \
		-c:v libx264 -pix_fmt yuv420p \
		video.mp4
		#-c:v h264_nvenc -preset p6 -tune hq -b:v 10M -pix_fmt yuv420p \

.PHONY: build run-raw run-with-vis
