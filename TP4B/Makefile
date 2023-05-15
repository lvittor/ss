
INPUT_FILE_PATH := TP4B/data/input.txt
OUTPUT_FILE_PATH := TP4B/data/output.txt
CAPTURE_DIR := 
RUN_ARGS :=
MAX_TIME :=
PRECISION := F64
FEATURES :=
MIN_BALL_AMOUNT :=
DELTA_TIME_N := 5
EVERY := 1000

ifeq "$(PRECISION)" "F64"
	FEATURES := "use_f64"
else
	FEATURES := "use_f32"
endif
MAX_TIME_ARG := $(if $(MAX_TIME),--max-duration $(MAX_TIME))
CAPTURE_DIR_ARG := $(if $(CAPTURE_DIR),--capture-directory $(CAPTURE_DIR))
MIN_BALL_AMOUNT_ARG := $(if $(MIN_BALL_AMOUNT),--min-ball-amount $(MIN_BALL_AMOUNT))


build:
	make -C .. -f Makefile.rust build PACKAGE=tp4b "FEATURE_ARGS=--no-default-features --features=$(FEATURES)"

run-with-vis: build
	make -C .. -f Makefile.rust -s run-raw BIN=simulation \
		ARGS="--input $(INPUT_FILE_PATH) $(MAX_TIME_ARG) --delta-time-n=$(DELTA_TIME_N) every $(EVERY) --last" | \
	make -C .. -f Makefile.rust run-raw BIN=visualization \
		ARGS="--input $(INPUT_FILE_PATH) --output /dev/stdin --no-holes $(CAPTURE_DIR_ARG)"

run-with-vis-2: build
	make -C .. -f Makefile.rust -s run-raw BIN=simulation \
		ARGS="--input $(INPUT_FILE_PATH) $(MAX_TIME_ARG) $(MIN_BALL_AMOUNT_ARG) --delta-time-n=5 --with-holes every 1000 --last" | \
	make -C .. -f Makefile.rust run-raw BIN=visualization \
		ARGS="--input $(INPUT_FILE_PATH) --output /dev/stdin $(CAPTURE_DIR_ARG)"

run-raw:
	make -C .. -f Makefile.rust -s run-raw PACKAGE=tp4b ARGS="$(RUN_ARGS)"

#generate-precision-diff-video:
	#make run-with-vis USE_DOCKER=FALSE PRECISION=F32 CAPTURE_DIR=TP4B/f32_capture
	#make run-with-vis USE_DOCKER=FALSE PRECISION=F64 CAPTURE_DIR=TP4B/f64_capture
	#ffmpeg \
		#-framerate 60 -pattern_type glob -i 'f64_capture/*.png' \
		#-framerate 60 -pattern_type glob -i 'f32_capture/*.png' \
		#-filter_complex "\
			#[1:v]chromakey=0x305a4a:0.01:0.1[ckout]; \
			#[0:v][ckout]blend=all_mode='overlay':all_opacity=0.7,split=3 [mixed0][mixed1][mixed2]; \
			#[mixed0]trim=end=2.4[v0]; \
			#[mixed1]trim=start=2.4:end=5, \
				#drawtext=text='x0.25':x=(w-text_w-20):y=(h-text_h-20):fontsize=120:fontcolor=white, \
				#setpts=PTS-STARTPTS,setpts=PTS*4[v1]; \
			#[mixed2]trim=start=5:end=10, \
				#drawtext=text='x1':x=(w-text_w-20):y=(h-text_h-20):fontsize=120:fontcolor=white, \
				#setpts=PTS-STARTPTS,setpts=PTS[v2]; \
			#[v0][v1][v2]concat=n=3" \
		#-c:v h264_nvenc -preset p6 -tune hq -b:v 10M -pix_fmt yuv420p \
		#diff.mp4
		##-c:v libx264 -pix_fmt yuv420p \

generate-video1:
	make run-with-vis USE_DOCKER=FALSE CAPTURE_DIR=TP4B/capture_k_2 DELTA_TIME_N=2 EVERY=1
	ffmpeg\
		-framerate 100 -pattern_type glob -i 'capture_k_2/*.png'\
		-filter_complex "\
			[0:v]trim=start=0:end=0.5[v0]; \
			[0:v]trim=start=0.5:end=1.5, \
				drawtext=text='x0.1':x=(w-text_w-20):y=(h-text_h-20):fontsize=120:fontcolor=white, \
				setpts=PTS-STARTPTS,setpts=PTS*10[v1]; \
			[v0][v1]concat=n=2" \
		-c:v h264_nvenc -preset p6 -tune hq -b:v 10M -pix_fmt yuv420p capture_k_2.mp4

generate-video2:
	make run-with-vis-2 USE_DOCKER=FALSE CAPTURE_DIR=TP4B/capture_y48 INPUT_FILE_PATH=TP4B/data/input48.txt
	ffmpeg\
		-framerate 100 -pattern_type glob -i 'capture_y48/*.png'\
		-c:v h264_nvenc -preset p6 -tune hq -b:v 10M -pix_fmt yuv420p capture_y48.mp4

.PHONY: build run-raw run-with-vis run-with-vis2
