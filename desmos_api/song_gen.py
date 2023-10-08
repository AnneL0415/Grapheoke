#!/usr/bin/env python3
from math import log

framing = 1000

programmed_time = 160
tpb = 480

def mid_to_hz(d):
	# Formula from Wikipedia
	return (2 ** ((d - 69) / 12)) * 440

def hz_to_des(h):
	return log(h/27)/log(273)*framing

def mid_to_des(d):
	return hz_to_des(mid_to_hz(d))



# python3 graph.py | xclip -selection c
def generate_piecewise_song(midi_file, track=1, note_count=500, octave_shift=22):
	from sys import argv, stderr
	from mido import MidiFile
	import mido as md

	# Transpose
	tp = octave_shift
	# Length
	lth = note_count
	# Channel
	ch = track
	# Filename
	filename = midi_file
	mid = MidiFile(filename, clip=True)
	notes = []

	time = 0
	absTime = 0
	note = 0
	start = 0

	theoreticalTime = mid.length
	

	note_on = False

	for msg in mid.tracks[ch][:lth]:
		time+=msg.time
		absTime+=md.tick2second(msg.time,tpb,500000)
		if(time<md.second2tick(programmed_time,tpb,500000)):
			if not note_on:
				if msg.type == "note_on" and msg.velocity > 0:
					note_on = True
					note = msg.note
					start = time
			elif note_on:
				if msg.type == "note_off" or (msg.type == "note_on" and msg.velocity == 0):
					if msg.note == note:
						note_on = False
						notes.append((start, time, note))
	s_t = 0
	time = notes[-1][1]
	# time -= s_t
	time /= framing
	#time = 1

	# Transposed to fit to scale
	#notes = [((a-s_t)/time, (b-s_t)/time, mid_to_des(((n + tp) % 12) + 72)) for a, b, n in notes]
	# As-is
	notes = [((a-s_t)/time, (b-s_t)/time, mid_to_des(n + tp)) for a, b, n in notes]

	formula = "y=\\left\\{" + ",".join([f"{a:.2f}\\le x\\le{b:.2f}:{n:.2f}" for a, b, n in notes]) + ", 0 \\le \\ x \\ 1000: 0 \\right\\}"

	return formula

# if __name__ == "__main__":
# 	from sys import argv, stderr
# 	from mido import MidiFile
# 	generate_piecewise_song(argv[1],int(argv[2]),int(argv[3]),int(argv[4]))