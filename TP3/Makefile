
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
		-filter_complex "\
			[1:v]chromakey=0x305a4a:0.01:0.1[ckout];\
			[0:v][ckout]blend=all_mode='overlay':all_opacity=0.7 [out]" \
		-map "[out]" \
		-c:v libx264 -pix_fmt yuv420p \
		video.mp4
		#-c:v h264_nvenc -preset p6 -tune hq -b:v 10M -pix_fmt yuv420p \

generate-speedup-video:
	ffmpeg\
		-framerate 60 -pattern_type glob -i '../sample_video/*.png'\
		-filter_complex "\
			[0:v]trim=start=0:end=5[v0]; \
			[0:v]trim=start=5:end=30, \
				drawtext=text='x5':x=(w-text_w-20):y=(h-text_h-20):fontsize=120:fontcolor=white, \
				setpts=PTS-STARTPTS,setpts=PTS/5[v1]; \
			[0:v]trim=start=30, \
				drawtext=text='x10':x=(w-text_w-20):y=(h-text_h-20):fontsize=120:fontcolor=white, \
				setpts=PTS-STARTPTS,setpts=PTS/10[v2]; \
			[v0][v1][v2]concat=n=3" \
		-c:v h264_nvenc -preset p6 -tune hq -b:v 10M -pix_fmt yuv420p video_speed.mp4

.PHONY: build run-raw run-with-vis
